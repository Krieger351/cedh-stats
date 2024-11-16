use crate::cache::Cache;
use anyhow::anyhow;

mod commander_entries;
mod top_commanders;
mod average;
mod id_win_rate;
mod deck_list;
mod ids_with_decklist;
mod all_cards;
mod card_list_map;
mod id_deck_list_map;
mod top_decks;
use crate::data_structures::Commander;
use anyhow::Result;
pub use top_decks::TopDeckMethod;

pub struct Store {
    pub commander_name: Commander,
    cache: Cache,
}

impl Store {
    pub async fn new(commander_name: &Commander) -> Result<Store> {
        let store = Store {
            commander_name: commander_name.clone(),
            cache: Cache::new(commander_name),
        };

        match &store.top_commanders().await?.get(&store.commander_name) {
            Some(_) => Ok(store),
            _ => Err(anyhow!("Invalid commander"))
        }
    }
}

