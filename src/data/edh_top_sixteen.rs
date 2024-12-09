use crate::data_types::deck_entry::DeckEntry;
use crate::data_types::deck_entry_list::DeckEntryList;
use crate::types::commander::Commander;
use crate::types::deck_id::DeckId;
use crate::types::win_rate::WinRate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommanderEntry {
    #[serde(alias = "decklist")]
    pub deck_list: Option<String>,
    #[serde(alias = "winRate")]
    pub win_rate: Option<f64>,
}

impl CommanderEntry {
    fn is_moxfield(self: &Self) -> bool {
        self.deck_list.is_some() && self.deck_list.as_ref().unwrap().contains("moxfield.com")
    }
    pub fn is_valid(self: &Self) -> bool {
        self.deck_list.is_some() && self.win_rate.is_some() && self.is_moxfield()
    }

    pub fn get_id(self: &Self) -> Option<DeckId> {
        if self.is_moxfield() {
            DeckId::from_moxfield(self.deck_list.clone().unwrap())
        } else {
            Some(self.deck_list.as_ref().unwrap().parse().unwrap())
        }
    }

    pub fn into_deck_entry(self) -> Option<DeckEntry> {
        if self.is_valid() {
            Some(DeckEntry::new(self.get_id().unwrap(), WinRate::new(self.win_rate.unwrap()).unwrap()))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommanderEntries(pub Vec<CommanderEntry>);

impl CommanderEntries {
    pub fn iter(&self) -> std::slice::Iter<'_, CommanderEntry> {
        self.0.iter()
    }
    pub fn into_deck_entry_list(self) -> DeckEntryList {
        self.0.into_iter().filter_map(|x| x.into_deck_entry()).collect::<DeckEntryList>()
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct SerdeEntries {
    entries: CommanderEntries,

}
#[derive(Serialize, Deserialize, Debug)]
struct SerdeCommander {
    commander: SerdeEntries,
}
#[derive(Serialize, Deserialize, Debug)]
struct SerdeResponse {
    data: SerdeCommander,
}

pub struct EdhTopSixteen {}

impl EdhTopSixteen {
    pub async fn get_commanders() -> anyhow::Result<HashSet<Commander>> {
        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct CommanderNamesWrapper {
            #[serde(alias = "commanderNames")]
            commander_names: HashSet<Commander>,
        }
        #[derive(Serialize, Deserialize, Debug, Clone)]
        struct ClientResponse {
            data: CommanderNamesWrapper,
        }
        let client = reqwest::Client::new();

        let body = json!({
            "query":"query Commanders { commanderNames}",
        });

        let response = client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        Ok(response.json::<ClientResponse>().await?.data.commander_names)
    }
    pub async fn get_entries(commander: &Commander, after) -> anyhow::Result<()> {
        let client = reqwest::Client::new();

        let body = json!({
            "query": r#"query Commanders($name: String!, $after: String, $filters: EntriesFilter) {
  commander(name: $name) {
    entries(after: $after, filters: $filters) {
      edges {
        cursor
        node {
          decklist
          maindeck {
            name
          }
          winRate
        }
      }
      pageInfo {
        hasNextPage
        endCursor
      }
    }
  }
}"#,
            "variables": {
                  "name": "Kinnan, Bonder Prodigy",
                  "after": null,
                  "filters": {
                    "minEventSize": 64,
                    "timePeriod": "SIX_MONTHS"
                  },
            }
        });

        let response = client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        let body = response.text().await?;

        #[derive(Serialize, Deserialize, Debug)]
        struct SerdeEdges {}
        #[derive(Serialize, Deserialize, Debug)]
        struct SerdePageInfo {
            #[serde(alias = "hasNextPage")]
            has_next_page: bool,
            #[serde(alias = "endCursor")]
            end_cursor: String,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct SerdeEntries {
            #[serde(alias = "pageInfo")]
            page_info: SerdePageInfo,
        }
        #[derive(Serialize, Deserialize, Debug)]
        struct SerdeCommander {
            entries: SerdeEntries,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct SerdeData {
            commander: SerdeCommander,
        }

        #[derive(Serialize, Deserialize, Debug)]
        struct SerdeResponse {
            data: SerdeData,
        }

        // let data = serde.deserde.json::<SerdeResponse>().await?.data.commander.entries;
        let data = serde_json::from_str::<SerdeResponse>(&body)?;
        println!("{data:?}");

        Ok(())
    }
}