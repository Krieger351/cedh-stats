use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
#[repr(transparent)]
pub struct Card(String);

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Card {
    pub fn from_string(card_name: String) -> Self {
        Card(card_name)
    }
}