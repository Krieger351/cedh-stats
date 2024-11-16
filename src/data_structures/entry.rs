use crate::data_structures::deck_id::DeckId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(alias = "decklist")]
    pub deck_list: Option<String>,
    #[serde(alias = "winRate")]
    pub win_rate: Option<f32>,
}

impl Entry {
    fn is_moxfield(self: &Self) -> bool {
        self.deck_list.is_some() && self.deck_list.as_ref().unwrap().contains("moxfield.com")
    }
    pub fn is_valid(self: &Self) -> bool {
        self.deck_list.is_some() && self.win_rate.is_some() && self.is_moxfield()
    }

    pub fn get_id(self: &Self) -> Option<DeckId> {
        if self.is_moxfield() {
            DeckId::from_moxfield(self.deck_list.clone().unwrap())
        } else {
            Some(DeckId::new(self.deck_list.as_ref().unwrap().to_string()))
        }
    }
}