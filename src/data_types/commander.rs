use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
#[repr(transparent)]
pub struct Commander(String);

impl Display for Commander {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_lowercase().replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '/', '\\'][..], "").replace(" ", "_"))
    }
}


impl FromStr for Commander {
    type Err = String;

    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|e| format!("{e}"))?))
    }
}