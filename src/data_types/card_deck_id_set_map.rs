use crate::data_types::deck_id_set::DeckIdSet;
use crate::types::card::Card;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CardDeckIdSetMap(HashMap<Card, DeckIdSet>);

impl CardDeckIdSetMap {
    pub fn new() -> Self {
        CardDeckIdSetMap(HashMap::new())
    }

    pub fn insert(&mut self, k: Card, v: DeckIdSet) -> Option<DeckIdSet> {
        self.0.insert(k, v)
    }

    pub fn get_mut(&mut self, k: &Card) -> Option<&mut DeckIdSet> {
        self.0.get_mut(k)
    }

    pub fn iter(&self) -> Iter<'_, Card, DeckIdSet> {
        self.0.iter()
    }
}