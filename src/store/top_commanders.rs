use crate::data_structures::Commander;
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


async fn fetch_top_commanders() -> Result<HashSet<Commander>> {
    let client = reqwest::Client::new();
    Ok(client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(r#"{"query": "query Query {commanderNames}"}"#)
        .send()
        .await?
        .json::<Response>().await?.data.commander_names)
}

impl Store {
    pub async fn top_commanders(self: &Self) -> Result<HashSet<Commander>> {
        if let Ok(data) = self.cache.read("top-commanders").await {
            Ok(data)
        } else {
            let data = fetch_top_commanders().await?;
            self.cache.write("top_commanders", &data).await?;
            Ok(data)
        }
    }
}