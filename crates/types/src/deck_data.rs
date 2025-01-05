use crate::card_list::CardList;
use crate::deck_id::DeckId;
use crate::win_rate::WinRate;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckData {
    id: DeckId,
    list: CardList,
    win_rate: WinRate,
}

impl DeckData {
    pub fn new(id: Option<DeckId>, list: Option<CardList>, win_rate: Option<WinRate>) -> Option<Self>
    {
        Some(DeckData { id: id?, list: list?, win_rate: win_rate? })
    }

    pub fn id(&self) -> &DeckId {
        &self.id
    }
    pub fn list(&self) -> &CardList {
        &self.list
    }
    pub fn win_rate(&self) -> &WinRate {
        &self.win_rate
    }
}