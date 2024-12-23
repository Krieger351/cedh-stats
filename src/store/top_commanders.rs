use crate::cache::{Cache, Cacheable, CommanderCache};
use crate::remote_data;
use crate::store::Store;
use crate::types::commander::Commander;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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
        remote_data::edh_top_sixteen::get_commanders().await
    }

    fn cache_file_path(&self) -> String {
        "top_commanders".to_string()
    }
}

impl Store<'_> {
    pub async fn fetch_top_commanders(&self) -> Result<HashSet<Commander>> {
        TopCommandersReader().compute().await
    }
    pub async fn top_commanders(&self) -> Result<HashSet<Commander>> {
        TopCommandersReader().load(&Cache::default()).await
    }
}