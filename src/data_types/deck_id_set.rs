use crate::data_types::deck_id::DeckId;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::Iter;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckIdSet(HashSet<DeckId>);

impl DeckIdSet {
    pub fn insert(&mut self, id: DeckId) -> bool {
        self.0.insert(id)
    }
    pub fn new() -> DeckIdSet {
        DeckIdSet(HashSet::new())
    }

    pub fn iter(&self) -> Iter<'_, DeckId> {
        self.0.iter()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn contains(&self, id: &DeckId) -> bool {
        self.0.contains(id)
    }
}

impl FromIterator<DeckId> for DeckIdSet {
    fn from_iter<T: IntoIterator<Item=DeckId>>(iter: T) -> Self {
        DeckIdSet(HashSet::<DeckId>::from_iter(iter))
    }
}