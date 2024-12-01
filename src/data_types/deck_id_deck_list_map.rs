use crate::data_types::deck_id::DeckId;
use crate::data_types::deck_id_set::DeckIdSet;
use crate::data_types::deck_list::DeckList;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{IntoIter, Iter, Keys, Values};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeckIdDeckListMap(HashMap<DeckId, DeckList>);

impl DeckIdDeckListMap {
    pub(crate) fn len(&self) -> usize {
        self.0.len()
    }
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
    pub fn values(&self) -> Values<'_, DeckId, DeckList> {
        self.0.values()
    }

    pub fn get(&self, id: &DeckId) -> Option<&DeckList> {
        self.0.get(id)
    }

    pub fn retain_by_key(&mut self, ids: &DeckIdSet) {
        self.0.retain(|x, x1| ids.contains(x))
    }

    pub fn into_all_cards(self) -> DeckList {
        let mut list = DeckList::new();
        for value in self.values() {
            list.extend(value.clone());
        }
        list
    }
}
impl IntoIterator for DeckIdDeckListMap {
    type Item = (DeckId, DeckList);
    type IntoIter = IntoIter<DeckId, DeckList>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<(DeckId, DeckList)> for DeckIdDeckListMap {
    fn from_iter<T: IntoIterator<Item=(DeckId, DeckList)>>(iter: T) -> Self {
        DeckIdDeckListMap(HashMap::from_iter(iter))
    }
}