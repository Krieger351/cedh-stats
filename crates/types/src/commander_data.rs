use crate::card_list::CardList;
use crate::win_rate::WinRate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommanderData {
    win_rate: WinRate,
    card_list: CardList,
}