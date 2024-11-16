use crate::data_structures::Entries;
use crate::store::Store;
use anyhow::Result;
use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug)]
struct SerdeEntries {
    entries: Entries,

}
#[derive(Serialize, Deserialize, Debug)]
struct SerdeCommander {
    commander: SerdeEntries,
}
#[derive(Serialize, Deserialize, Debug)]
struct SerdeResponse {
    data: SerdeCommander,
}


async fn fetch_commander_entries(commander_name: &str) -> Result<Entries> {
    let client = reqwest::Client::new();

    let body = json!({
        "query":"query CommanderEntries($name: String!, $filters: EntryFilters) {  commander(name: $name) { entries(filters: $filters) {decklist\nwinRate}}}",
        "variables": {
            "name": commander_name,
            "filters": {
                "minSize":64,
                "minDate":"2024/01/01"
            }
        }
    });

    Ok(client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?.json::<SerdeResponse>().await?.data.commander.entries)
}
impl Store {
    pub async fn commander_entries(self: &Self) -> Result<Entries> {
        if let Ok(data) = self.cache.read_commander("commander-entries").await {
            Ok(data)
        } else {
            let data = fetch_commander_entries(&self.commander_name).await?;
            self.cache.write_commander("commander-entries", &data).await?;
            Ok(data)
        }
    }
}
