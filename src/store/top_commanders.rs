use crate::cache::{Cache, Cacheable};
use crate::data_types::commander::Commander;
use crate::store::Store;
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
impl Cacheable<'_, HashSet<Commander>> for TopCommandersReader {
    type C<'c> = Cache;

    async fn compute(&self) -> Result<HashSet<Commander>> {
        let client = reqwest::Client::new();
        Ok(client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
            .body(r#"{"query": "query Query {commanderNames}"}"#)
            .send()
            .await?
            .json::<Response>().await?.data.commander_names)
    }

    fn cache_file_path(&self) -> String {
        "top_commanders".to_string()
    }
}

impl Store<'_> {
    pub async fn top_commanders(&self) -> Result<HashSet<Commander>> {
        TopCommandersReader().load_or_compute(&self.cache).await
    }
}