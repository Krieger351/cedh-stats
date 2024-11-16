use crate::data_structures::DeckId;
use crate::data_structures::DeckList;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct IdDeckListMap(HashMap<DeckId, DeckList>);

impl IdDeckListMap {
    pub fn new() -> IdDeckListMap {
        IdDeckListMap(HashMap::<DeckId, DeckList>::new())
    }

    pub fn insert(&mut self, k: DeckId, v: DeckList) -> Option<DeckList> {
        self.0.insert(k, v)
    }

    pub fn iter(&self) -> Iter<DeckId, DeckList> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<'_, DeckId, DeckList> {
        self.0.keys()
    }

    pub fn get(&self, id: &DeckId) -> Option<&DeckList> {
        self.0.get(id)
    }
}
