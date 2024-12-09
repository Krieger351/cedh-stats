use crate::data_types::deck_id_set::DeckIdSet;
use crate::types::deck_id::DeckId;
use crate::types::win_rate::WinRate;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Keys;
use std::collections::hash_map::{IntoIter, Values};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckIdWinRateMap(HashMap<DeckId, WinRate>);

impl FromIterator<(DeckId, WinRate)> for DeckIdWinRateMap {
    fn from_iter<T: IntoIterator<Item=(DeckId, WinRate)>>(iter: T) -> Self {
        DeckIdWinRateMap(HashMap::from_iter(iter))
    }
}

impl IntoIterator for DeckIdWinRateMap {
    type Item = (DeckId, WinRate);
    type IntoIter = IntoIter<DeckId, WinRate>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl DeckIdWinRateMap {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    fn mean(&self) -> WinRate {
        self.0.values().map(|x| x * x).sum::<WinRate>() / self.0.values().len()
    }

    fn standard_dev(&self) -> WinRate {
        let mean = self.mean();
        self.0.values().map(|x| (x - &mean).powi(2)).sum::<WinRate>() / self.0.values().len() as f64
    }

    pub fn into_top_decks_by_positive(self) -> Self {
        self.0.into_iter().filter(|(_, val)| val > &0.0).collect::<Self>()
    }

    pub fn into_top_decks_by_z_score(self) -> Self {
        let mean = self.mean();
        let standard_dev = self.standard_dev();
        self.0.into_iter().filter(|(_, val)| ((val - &mean) / standard_dev).abs() <= 3.0).collect::<Self>()
    }

    pub fn into_top_decks_by_percent(self) -> Self {
        self.0.into_iter().filter(|x| { x.1 > 0.25 }).collect::<Self>()
    }


    pub fn into_top_decks_by_quartile(self) -> Self {
        let mut sorted = self.0.values().clone().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let len = sorted.len();
        let start = (len as f64 * 0.75).ceil() as usize;
        let top_quartile_val = sorted[start].clone();

        self.0.into_iter().filter(|(_, x)| x >= &top_quartile_val).collect::<Self>()
    }

    pub fn new() -> Self {
        DeckIdWinRateMap(HashMap::<DeckId, WinRate>::new())
    }

    pub fn insert(&mut self, k: DeckId, v: WinRate) -> Option<WinRate> {
        self.0.insert(k, v)
    }
    pub fn values(&self) -> Values<'_, DeckId, WinRate> {
        self.0.values()
    }
    pub fn keys(&self) -> Keys<'_, DeckId, WinRate> {
        self.0.keys()
    }
    pub fn get(&self, p0: &DeckId) -> Option<&WinRate> {
        self.0.get(p0)
    }

    pub fn average_win_rate(&self) -> WinRate {
        self.0.values().sum::<WinRate>() / self.0.values().len()
    }

    pub fn retain_by_deck_id_set(&mut self, ids: &DeckIdSet) {
        self.0.retain(|x, x1| ids.contains(x))
    }
}
