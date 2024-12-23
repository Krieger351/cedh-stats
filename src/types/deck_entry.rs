use crate::types::deck_id::DeckId;
use crate::types::win_rate::WinRate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckEntry {
    id: DeckId,
    win_rate: WinRate,
}

impl From<(DeckId, WinRate)> for DeckEntry {
    fn from((id, win_rate): (DeckId, WinRate)) -> Self {
        DeckEntry { id, win_rate }
    }
}
impl Into<(DeckId, WinRate)> for DeckEntry {
    fn into(self) -> (DeckId, WinRate) {
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
    pub fn id(&self) -> &DeckId {
        &self.id
    }
    pub fn win_rate(&self) -> &WinRate {
        &self.win_rate
    }
}