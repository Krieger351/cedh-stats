use crate::data_structures::DeckId;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Keys;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct IdWinRate(HashMap<DeckId, f32>);

impl IdWinRate {}

impl IdWinRate {
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