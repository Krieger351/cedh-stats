use crate::data_structures::{Card, DeckId};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardListIdMap(HashMap<Card, HashSet<DeckId>>);

impl CardListIdMap {
    pub fn new() -> Self {
        CardListIdMap(HashMap::<Card, HashSet<DeckId>>::new())
    }

    pub fn insert(&mut self, k: Card, v: HashSet<DeckId>) -> Option<HashSet<DeckId>> {
        self.0.insert(k, v)
    }

    pub fn get_mut(&mut self, k: &Card) -> Option<&mut HashSet<DeckId>> {
        self.0.get_mut(k)
    }
}