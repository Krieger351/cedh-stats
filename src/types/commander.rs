use crate::cache::{CommanderCache, FileController};
use crate::data::edh_top_sixteen::CommanderEntries;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialOrd, PartialEq, Eq)]
#[repr(transparent)]
pub struct Commander(String);

impl Commander {
    pub async fn entries(&self) -> anyhow::Result<()> {
        let cache = CommanderCache::new(self);
        let entries = cache.read::<CommanderEntries>("commander-entries").await?;

        print!("{:?}", entries);

        Ok(())
    }
}
impl Display for Commander {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}


impl FromStr for Commander {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}