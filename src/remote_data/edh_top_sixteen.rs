use crate::types::card::Card;
use crate::types::commander::Commander;
use crate::types::deck_data::DeckData;
use crate::types::deck_data_list::DeckDataList;
use crate::types::deck_id::DeckId;
use crate::types::win_rate::WinRate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClientResponse<T> {
    data: T,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommanderNamesWrapper {
    #[serde(alias = "commanderNames")]
    commander_names: HashSet<Commander>,
}
pub async fn get_commanders() -> anyhow::Result<HashSet<Commander>> {
    let client = reqwest::Client::new();

    let body = json!({
            "query":"query Commanders { commanderNames}",
        });

    let response = client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?;

    Ok(response.json::<ClientResponse<CommanderNamesWrapper>>().await?.data.commander_names)
}

#[derive(Deserialize, Debug)]
struct SerdeEdgeCard {
    name: Card,
}
#[derive(Deserialize, Debug)]
struct SerdeEdgeNode {
    #[serde(alias = "winRate")]
    win_rate: Option<WinRate>,
    #[serde(alias = "maindeck")]
    main_deck: Vec<SerdeEdgeCard>,
    #[serde(alias = "decklist")]
    deck_list: String,
}

impl SerdeEdgeNode {
    fn into_deck_data(self) -> Option<DeckData> {
        DeckData::new(DeckId::from_moxfield(self.deck_list), Some(self.main_deck.into_iter().map(|x| { x.name }).collect::<Vec<_>>()), self.win_rate)
    }
}

#[derive(Deserialize, Debug)]
struct SerdeEdge {
    node: SerdeEdgeNode,
}
#[derive(Deserialize, Debug)]
struct SerdePageInfo {
    #[serde(alias = "hasNextPage")]
    has_next_page: bool,
    #[serde(alias = "endCursor")]
    end_cursor: String,
}
#[derive(Deserialize, Debug)]
struct SerdeEntries {
    edges: Vec<SerdeEdge>,
    #[serde(alias = "pageInfo")]
    page_info: SerdePageInfo,
}
#[derive(Deserialize, Debug)]
struct SerdeCommander {
    entries: SerdeEntries,
}

#[derive(Deserialize, Debug)]
struct SerdeData {
    commander: SerdeCommander,
}

#[derive(Deserialize, Debug)]
struct SerdeResponse {
    data: SerdeData,
}


pub async fn get_entries_recursive(commander: &Commander, after: Option<String>) -> reqwest::Result<SerdeResponse> {
    let client = reqwest::Client::new();

    let body = json!({
            "query": r#"query Commanders($name: String!, $after: String, $filters: EntriesFilter) {
  commander(name: $name) {
    entries(after: $after, filters: $filters) {
      edges {
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
                  "name": commander,
                  "after": after,
                  "filters": {
                    "minEventSize": 64,
                    "timePeriod": "SIX_MONTHS"
                  },
            }
        });

    client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .json().await
}

pub async fn get_entries(commander: &Commander) -> anyhow::Result<DeckDataList> {
    let mut has_next = true;
    let mut next = None;

    let mut deck_data_list = DeckDataList::new();
    while has_next {
        let response = get_entries_recursive(commander, next).await?;
        has_next = response.data.commander.entries.page_info.has_next_page;
        next = Some(response.data.commander.entries.page_info.end_cursor);

        deck_data_list.extend(response.data.commander.entries.edges.into_iter().filter_map(|x| x.node.into_deck_data()));
    }
    Ok(deck_data_list)
}
