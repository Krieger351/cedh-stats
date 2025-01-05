use crate::Store;
use anyhow::Result;
use cache::{Cache, Cacheable, CommanderCache};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use top_sixteen;
use types::commander::Commander;

#[derive(Serialize, Deserialize, Debug)]
pub struct TopCommanders {
    #[serde(rename(deserialize = "commanderNames"))]
    commander_names: HashSet<Commander>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: TopCommanders,
}


struct TopCommandersReader();
impl<'a> Cacheable<'a, HashSet<Commander>> for TopCommandersReader {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<HashSet<Commander>> {
        top_sixteen::get_commanders().await
    }

    fn cache_file_path(&self) -> String {
        "top_commanders".to_string()
    }
}

impl Store<'_> {
    pub async fn fetch_top_commanders(&self) -> Result<HashSet<Commander>> {
        TopCommandersReader().load_or_compute(&Cache::new()).await
    }
    pub async fn top_commanders(&self) -> Result<HashSet<Commander>> {
        TopCommandersReader().load(&Cache::default()).await
    }
}