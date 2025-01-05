use crate::card::Card;
use crate::card_set::CardSet;
use crate::deck_data::DeckData;
use crate::deck_id_set::DeckIdSet;
use crate::win_rate::WinRate;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::{Index, RangeFull};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Debug, Clone, ValueEnum)]
pub enum TopDeckMethod {
    Percent,
    Quartile,
    ZScore,
    Positive,
}
impl Display for TopDeckMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DeckDataList(Vec<DeckData>);

impl DeckDataList {
    pub fn new() -> Self {
        DeckDataList(Vec::new())
    }
    pub fn iter(&self) -> Iter<'_, DeckData> {
        self.0.iter()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn mean(&self) -> WinRate {
        self.0.iter().map(|x| x.win_rate()).sum::<WinRate>() / self.0.len()
    }

    fn standard_dev(&self) -> WinRate {
        let mean = self.mean();
        self.0.iter().map(|x| (x.win_rate() - &mean).powi(2)).sum::<WinRate>() / self.0.len() as f64
    }

    pub fn into_top_decks_by_positive(self) -> Self {
        self.0.into_iter().filter(|val| val.win_rate() > &0.0).collect::<Self>()
    }

    pub fn into_top_decks_by_z_score(self) -> Self {
        let mean = self.mean();
        let standard_dev = self.standard_dev();
        self.0.into_iter().filter(|val| ((val.win_rate() - &mean) / standard_dev).abs() <= 3.0).collect::<Self>()
    }

    pub fn into_top_decks_by_percent(self) -> Self {
        self.0.into_iter().filter(|x| { x.win_rate() > &0.25 }).collect::<Self>()
    }

    pub fn into_top_decks_by_quartile(self) -> Self {
        let mut sorted: Vec<_> = self.0.iter().map(|x1| x1.win_rate()).collect();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let len = sorted.len();
        let start = (len as f64 * 0.75).ceil() as usize;
        let top_quartile_val = sorted[start].clone();

        self.0.iter().filter(|x| x.win_rate() >= &top_quartile_val).cloned().collect::<Self>()
    }


    pub fn into_top_decks(self, method: &TopDeckMethod) -> Self {
        match method {
            TopDeckMethod::Percent => self.into_top_decks_by_percent(),
            TopDeckMethod::Quartile => self.into_top_decks_by_quartile(),
            TopDeckMethod::ZScore => self.into_top_decks_by_z_score(),
            TopDeckMethod::Positive => self.into_top_decks_by_positive(),
        }
    }

    pub fn into_top_decks_with_methods(self, methods: &[TopDeckMethod]) -> Self {
        let mut next = self.clone();
        for method in methods {
            next = next.into_top_decks(method);
        }
        next
    }

    pub fn win_rate_per_card(&self) -> HashMap<Card, WinRate> {
        let mut map = HashMap::new();
        for entry in self.iter() {
            let card_list = entry.list().into_iter();
            for card in card_list {
                map.entry(card).or_insert(HashSet::new()).insert(entry.id().clone());
            }
        }

        let mut averages = HashMap::new();

        for (card, set) in map {
            averages.insert(card.clone(), self.clone().pick_decks(&set).average());
        }

        averages
    }

    pub fn average(&self) -> WinRate {
        let mut sum = WinRate::from(0.).unwrap();
        let count = self.iter().len();

        for win_rate in self.iter() {
            sum += win_rate.win_rate();
        }
        sum / count
    }

    pub fn all_cards(&self) -> CardSet {
        let mut all_cards = CardSet::new();
        self.iter().for_each(|b| all_cards.extend(b.list().iter().cloned()));
        all_cards
    }

    pub fn pick_decks(self, sets: &DeckIdSet) -> DeckDataList {
        Self::from_iter(self.into_iter().filter(|x| sets.contains(x.id())))
    }

    pub fn common_cards(&self) -> CardSet {
        let mut cards = CardSet::from_iter(self.0[0].list().iter().cloned());
        for data in
            &self.0[1..] {
            cards = cards.intersection(&CardSet::from_iter(data.list().iter().cloned())).cloned().collect();
        }
        cards
    }
}
impl Default for TopDeckMethod {
    fn default() -> Self {
        Self::Positive
    }
}
impl IntoIterator for DeckDataList {
    type Item = DeckData;
    type IntoIter = IntoIter<DeckData>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<DeckData> for DeckDataList {
    fn extend<T: IntoIterator<Item=DeckData>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl FromIterator<DeckData> for DeckDataList {
    fn from_iter<T: IntoIterator<Item=DeckData>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Index<RangeFull> for DeckDataList {
    type Output = [DeckData];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}


