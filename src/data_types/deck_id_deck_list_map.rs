use crate::data_types::deck_id_set::DeckIdSet;
use crate::types::card_list::CardList;
use crate::types::deck_id::DeckId;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{IntoIter, Iter, Keys, Values};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeckIdDeckListMap(HashMap<DeckId, CardList>);

impl DeckIdDeckListMap {
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn new() -> DeckIdDeckListMap {
        DeckIdDeckListMap(HashMap::<DeckId, CardList>::new())
    }

    pub fn insert(&mut self, k: DeckId, v: CardList) -> Option<CardList> {
        self.0.insert(k, v)
    }

    pub fn iter(&self) -> Iter<DeckId, CardList> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<'_, DeckId, CardList> {
        self.0.keys()
    }
    pub fn values(&self) -> Values<'_, DeckId, CardList> {
        self.0.values()
    }

    pub fn get(&self, id: &DeckId) -> Option<&CardList> {
        self.0.get(id)
    }

    pub fn retain_by_key(&mut self, ids: &DeckIdSet) {
        self.0.retain(|x, x1| ids.contains(x))
    }

    pub fn into_all_cards(self) -> CardList {
        let mut list = CardList::new();
        for value in self.values() {
            list.extend(value.clone());
        }
        list
    }
}
impl IntoIterator for DeckIdDeckListMap {
    type Item = (DeckId, CardList);
    type IntoIter = IntoIter<DeckId, CardList>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(DeckId, CardList)> for DeckIdDeckListMap {
    fn from_iter<T: IntoIterator<Item=(DeckId, CardList)>>(iter: T) -> Self {
        DeckIdDeckListMap(HashMap::from_iter(iter))
    }
}