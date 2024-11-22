use crate::data_types::deck_entry::DeckEntry;
use crate::data_types::deck_id_win_rate_map::DeckIdWinRateMap;

pub struct DeckEntryList(Vec<DeckEntry>);

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
