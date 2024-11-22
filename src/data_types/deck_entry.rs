use crate::data_types::deck_id::DeckId;
use crate::data_types::win_rate::WinRate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeckEntry {
    pub id: DeckId,
    pub win_rate: WinRate,
}

impl DeckEntry {
    pub fn into_tuple(self) -> (DeckId, WinRate) {
        (self.id, self.win_rate)
    }
}

impl DeckEntry {
    pub fn new(id: DeckId, win_rate: WinRate) -> DeckEntry {
        DeckEntry {
            id,
            win_rate,
        }
    }
}