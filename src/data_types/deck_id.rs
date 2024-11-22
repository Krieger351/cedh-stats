use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct DeckId(String);

impl Display for DeckId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl DeckId {
    pub fn new(string: String) -> Self {
        DeckId(string)
    }
    pub fn from_moxfield(moxfield_url: String) -> Option<Self> {
        Some(DeckId(moxfield_url.replace("www.", "").replace("https://moxfield.com/decks/", "")))
    }
    pub fn from_str(str: &str) -> DeckId {
        DeckId(str.to_string())
    }
}