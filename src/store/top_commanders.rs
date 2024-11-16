use crate::store::Store;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TopCommanders {
    #[serde(rename(deserialize = "commanderNames"))]
    commander_names: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: TopCommanders,
}


async fn fetch_top_commanders() -> Result<Vec<String>> {
    let client = reqwest::Client::new();
    Ok(client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(r#"{"query": "query Query {commanderNames}"}"#)
        .send()
        .await?
        .json::<Response>().await?.data.commander_names)
}

impl Store {
    pub async fn top_commanders(self: &Self) -> Result<Vec<String>> {
        if let Ok(data) = self.cache.read("top-commanders").await {
            Ok(data)
        } else {
            let data = fetch_top_commanders().await?;
            self.cache.write("top_commanders", &data).await?;
            Ok(data)
        }
    }
}