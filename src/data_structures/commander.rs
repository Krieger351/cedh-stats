use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
pub struct Commander(String);

impl Display for Commander {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_lowercase().replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '/', '\\'][..], "").replace(" ", "_"))
    }
}

impl Commander {
    pub fn from_string(string: String) -> Self {
        Commander(string)
    }
}