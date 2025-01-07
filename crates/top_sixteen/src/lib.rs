use serde::{Deserialize, Serialize};
use serde_json::{from_str, json};
use std::collections::HashSet;
use types::card::Card;
use types::commander::Commander;
use types::deck_data::DeckData;
use types::deck_data_list::DeckDataList;
use types::deck_id::DeckId;
use types::win_rate::WinRate;

#[derive(Deserialize, Debug)]
struct Response<Data> {
    data: Data,
}

#[derive(Deserialize, Debug)]
struct EntriesData {
    commander: Entries<EntryNode>,
}


#[derive(Deserialize, Debug)]
struct TopSixteenEdgeCard {
    name: Card,
}

#[derive(Deserialize, Debug)]
struct EntryNode {
    #[serde(alias = "winRate")]
    win_rate: Option<WinRate>,
    #[serde(alias = "maindeck")]
    main_deck: Vec<TopSixteenEdgeCard>,
    #[serde(alias = "decklist")]
    deck_list: String,
    standing: usize,
}

impl EntryNode {
    fn into_deck_data(self) -> Option<DeckData> {
        DeckData::new(DeckId::from_moxfield(self.deck_list), Some(self.main_deck.into_iter().map(|x| { x.name }).collect::<Vec<_>>()), self.win_rate, Some(self.standing.into()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CommanderNode {
    name: Commander,
}

#[derive(Deserialize, Debug)]
struct Entries<Node> {
    entries: TopSixteenEntries<Node>,
}

#[derive(Deserialize, Debug)]
struct Edge<Node> {
    node: Node,
}

#[derive(Deserialize, Debug)]
struct TopSixteenEntries<Node> {
    edges: Vec<Edge<Node>>,
    #[serde(alias = "pageInfo")]
    page_info: PageInfo,
}

#[derive(Deserialize, Debug)]
struct PageInfo {
    #[serde(alias = "hasNextPage")]
    has_next_page: bool,
    #[serde(alias = "endCursor")]
    end_cursor: String,
}

#[derive(Deserialize, Debug)]
struct Commanders {
    #[serde(alias = "pageInfo")]
    page_info: PageInfo,
    edges: Vec<Edge<CommanderNode>>,
}

#[derive(Deserialize, Debug)]
struct CommanderData {
    commanders: Commanders,
}

async fn get_commanders_recursive(after: Option<String>) -> reqwest::Result<Response<CommanderData>> {
    let client = reqwest::Client::new();

    let body = json!({
            "query": r#"query Commanders($after: String) {
  commanders(after: $after) {
    edges {
      node {
        name
      }
    }
    pageInfo {
      hasNextPage
      endCursor
    }
  }
}"#,
            "variables": {
                  "after": after,
            }
        });

    let text = client.request(reqwest::Method::POST, "https://edhtop16.com/api/graphql").header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?
        .text().await?;
    Ok(from_str::<Response<CommanderData>>(&text).unwrap())
}

pub async fn get_commanders<'a>() -> anyhow::Result<HashSet<Commander>> {
    let mut has_next = true;
    let mut next = None;

    let mut commanders: HashSet<Commander> = HashSet::new();
    while has_next {
        let response = get_commanders_recursive(next).await?;
        has_next = response.data.commanders.page_info.has_next_page;
        next = Some(response.data.commanders.page_info.end_cursor);

        commanders.extend(response.data.commanders.edges.into_iter().filter_map(|x| Some(x.node.name)));
    }
    Ok(commanders)
}


async fn get_entries_recursive(commander: &Commander, after: Option<String>) -> reqwest::Result<Response<EntriesData>> {
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
          standing
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

