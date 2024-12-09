use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq, Eq, Hash)]
pub struct DeckId(String);

impl DeckId {
    pub fn as_moxfield_url(&self) -> String {
        format!("https://www.moxfield.com/decks/{}", self)
    }
}

impl Display for DeckId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl FromStr for DeckId {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}
impl DeckId {
    pub fn from_moxfield(moxfield_url: String) -> Option<Self> {
        Some(DeckId(moxfield_url.replace("www.", "").replace("https://moxfield.com/decks/", "")))
    }
}