use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::commander::Commander;
use crate::data_types::deck_entry::DeckEntry;
use crate::data_types::deck_entry_list::DeckEntryList;
use crate::data_types::deck_id::DeckId;
use crate::data_types::win_rate::WinRate;
use crate::store::Store;
use anyhow::Result;
use reqwest;
use serde;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
            Some(DeckId::new(self.deck_list.as_ref().unwrap().to_string()))
        }
    }

    pub fn into_deck_entry(self) -> Option<DeckEntry> {
        if self.is_valid() {
            Some(DeckEntry::new(self.get_id().unwrap(), WinRate::from_f64(self.win_rate.unwrap())))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[repr(transparent)]
pub struct CommanderEntries(Vec<CommanderEntry>);

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

struct AllCommanderEntriesReader<'a>(&'a Commander);

impl Cacheable<'_, CommanderEntries> for AllCommanderEntriesReader<'_> {
    type C<'a> = CommanderCache<'a>;
    async fn compute(&self) -> Result<CommanderEntries> {
        let client = reqwest::Client::new();

        let body = json!({
            "query":"query CommanderEntries($name: String!, $filters: EntryFilters) {  commander(name: $name) { entries(filters: $filters) {decklist\nwinRate}}}",
            "variables": {
                "name": self.0,
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

    fn cache_file_path(&self) -> String {
        "commander-entries".to_string()
    }
}
impl Store<'_> {
    pub async fn all_commander_entries(self: &Self) -> Result<CommanderEntries> {
        AllCommanderEntriesReader(self.commander).load_or_compute(&self.commander_cache).await
    }
}
