use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub struct Card(String);

impl Card {
    pub fn new(card_name: String) -> Self {
        Card(card_name)
    }
}