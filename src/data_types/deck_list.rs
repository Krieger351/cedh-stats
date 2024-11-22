use crate::data_types::card::Card;
use serde::{Deserialize, Serialize};
use std::collections::hash_set::Iter;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[repr(transparent)]
pub struct DeckList(HashSet<Card>);

impl IntoIterator for DeckList {
    type Item = Card;
    type IntoIter = std::collections::hash_set::IntoIter<Card>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Card> for DeckList {
    fn from_iter<T: IntoIterator<Item=Card>>(iter: T) -> Self {
        DeckList(HashSet::<Card>::from_iter(iter))
    }
}

impl Extend<Card> for DeckList {
    fn extend<T: IntoIterator<Item=Card>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl DeckList {
    pub fn new() -> Self {
        DeckList(HashSet::new())
    }

    pub fn into_hash_set(self) -> HashSet<Card> {
        self.0
    }

    pub fn iter(&self) -> Iter<'_, Card> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn intersection(&self, other: &Self) -> DeckList {
        DeckList(self.0.intersection(&other.0).cloned().collect::<HashSet<Card>>())
    }
    pub fn union(&self, other: &Self) -> DeckList {
        DeckList(self.0.union(&other.0).cloned().collect::<HashSet<Card>>())
    }
}