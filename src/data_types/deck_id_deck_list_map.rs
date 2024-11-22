use crate::data_types::deck_id::DeckId;
use crate::data_types::deck_list::DeckList;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Iter, Keys};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeckIdDeckListMap(HashMap<DeckId, DeckList>);

impl DeckIdDeckListMap {
    pub fn new() -> DeckIdDeckListMap {
        DeckIdDeckListMap(HashMap::<DeckId, DeckList>::new())
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
