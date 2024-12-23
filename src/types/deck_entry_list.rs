use crate::types::deck_entry::DeckEntry;
use crate::types::win_rate::WinRate;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::{Index, RangeFull};
use std::slice::{Iter, SliceIndex};

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
struct DeckEntryList(Vec<DeckEntry>);

impl DeckEntryList {
    pub fn push(&mut self, value: DeckEntry) {
        self.0.push(value)
    }
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn iter(&self) -> Iter<'_, DeckEntry> {
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
}

impl FromIterator<DeckEntry> for DeckEntryList {
    fn from_iter<T: IntoIterator<Item=DeckEntry>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Index<RangeFull> for DeckEntryList {
    type Output = [DeckEntry];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

