use crate::data_types::deck_list::DeckList;
use std::collections::HashSet;

pub struct DeckListSet(HashSet<DeckList>);

impl DeckListSet {
    pub fn len(&self) -> usize {
        self.0.len()
    }
}