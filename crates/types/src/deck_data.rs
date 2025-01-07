use crate::card_list::CardList;
use crate::deck_id::DeckId;
use crate::standing::Standing;
use crate::win_rate::WinRate;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeckData {
    id: DeckId,
    list: CardList,
    win_rate: WinRate,
    standing: Standing,
}

impl DeckData {
    pub fn new(id: Option<DeckId>, list: Option<CardList>, win_rate: Option<WinRate>, standing: Option<Standing>) -> Option<Self>
    {
        Some(DeckData { id: id?, list: list?, win_rate: win_rate?, standing: standing? })
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
    pub fn standing(&self) -> &Standing {
        &self.standing
    }
}