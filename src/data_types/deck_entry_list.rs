use crate::data_types::deck_entry::DeckEntry;
use crate::data_types::deck_id_win_rate_map::DeckIdWinRateMap;
use serde::{Deserialize, Serialize};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckEntryList(Vec<DeckEntry>);

impl DeckEntryList {
    pub fn iter(&self) -> Iter<'_, DeckEntry> {
        self.0.iter()
    }
}

impl DeckEntryList {
    pub fn into_deck_id_win_rate_map(self) -> DeckIdWinRateMap {
        self.0.into_iter().map(|x| x.into_tuple()).collect::<DeckIdWinRateMap>()
    }
}

impl FromIterator<DeckEntry> for DeckEntryList {
    fn from_iter<T: IntoIterator<Item=DeckEntry>>(iter: T) -> Self {
        DeckEntryList(Vec::<DeckEntry>::from_iter(iter))
    }
}

impl IntoIterator for DeckEntryList {
    type Item = DeckEntry;
    type IntoIter = IntoIter<DeckEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}