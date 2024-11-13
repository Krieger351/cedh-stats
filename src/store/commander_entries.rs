use crate::store::Store;
use anyhow::Result;
use reqwest;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    decklist: Option<String>,
    #[serde(rename(deserialize = "winRate"))]
    win_rate: Option<f32>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Entries {
    entries: Vec<Entry>,

}
#[derive(Serialize, Deserialize, Debug)]
struct Commander {
    commander: Entries,
}
#[derive(Serialize, Deserialize, Debug)]
struct Response {
    data: Commander,
}


async fn fetch_commander_entries(commander_name: &str) -> Result<Vec<Entry>> {
    let client = reqwest::Client::new();

    Ok(client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(r#"{"query":"query CommanderEntries($name: String!, $filters: EntryFilters) {  commander(name: $name) {\n    entries(filters: $filters) {\n      decklist\n      winRate\n    }\n  }\n}\n    ","variables":{"name":"Kinnan, Bonder Prodigy","filters":{"minSize":64,"minDate":"2024/01/01"}}}"#)
        .send()
        .await?.json::<Response>().await?.data.commander.entries)
}
impl Store {
    pub async fn commander_entries(self: &Self) -> Result<Vec<Entry>> {
        if let Ok(data) = self.cache.read_commander("commander-entries".to_string()).await {
            Ok(data)
        } else {
            let data = fetch_commander_entries(&self.commander_name).await?;
            self.cache.write_commander("commander-entries".to_string(), &data).await?;
            Ok(data)
        }
    }
}
