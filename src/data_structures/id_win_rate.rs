use crate::data_structures::DeckId;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Keys;
use std::collections::HashMap;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdWinRate(HashMap<DeckId, f32>);

impl FromIterator<(DeckId, f32)> for IdWinRate {
    fn from_iter<T: IntoIterator<Item=(DeckId, f32)>>(iter: T) -> Self {
        IdWinRate(HashMap::from_iter(iter))
    }
}

impl IdWinRate {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }

    fn mean(&self) -> f32 {
        self.0.values().map(|x| x * x).sum::<f32>() / self.0.values().len() as f32
    }

    fn standard_dev(&self) -> f32 {
        let mean = self.mean();
        self.0.values().map(|x| (x - mean).powi(2)).sum::<f32>() / self.0.values().len() as f32
    }

    pub(crate) fn into_top_decks_by_z_score(self) -> Self {
        let mean = self.mean();
        let standard_dev = self.standard_dev();
        self.0.into_iter().filter(|(_, val)| ((val - mean) / standard_dev).abs() <= 3.0).collect::<Self>()
    }

    pub(crate) fn into_top_decks_by_percent(self) -> Self {
        self.0.into_iter().filter(|x| { x.1 > 0.25 }).collect::<Self>()
    }


    pub(crate) fn into_top_decks_by_quartile(self) -> Self {
        let mut sorted = self.0.values().clone().collect::<Vec<_>>();
        sorted.sort_by(|a, b| b.partial_cmp(a).unwrap());
        let len = sorted.len();
        let start = (len as f32 * 0.75).ceil() as usize;
        let top_quartile_val = sorted[start].clone();

        self.0.into_iter().filter(|(_, x)| x >= &top_quartile_val).collect::<Self>()
    }

    pub(crate) fn new() -> Self {
        IdWinRate(HashMap::<DeckId, f32>::new())
    }

    pub(crate) fn insert(&mut self, k: DeckId, v: f32) -> Option<f32> {
        self.0.insert(k, v)
    }


    pub(crate) fn keys(&self) -> Keys<'_, DeckId, f32> {
        self.0.keys()
    }
    pub(crate) fn get(&self, p0: &DeckId) -> Option<&f32> {
        self.0.get(p0)
    }
}