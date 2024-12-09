use crate::types::card_list::CardList;
use crate::types::deck_id::DeckId;
use crate::types::win_rate::WinRate;

pub struct Deck {
    id: DeckId,
    list: CardList,
    win_rate: WinRate,
    author: Option<String>,
    name: Option<String>,
}

impl Deck {
    pub fn new(id: DeckId, list: CardList, win_rate: WinRate, author: Option<String>, name: Option<String>) -> Self {
        Deck { id, list, win_rate, author, name }
    }
}