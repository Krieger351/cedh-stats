use crate::cache::Cache;

mod commander_entries;
mod top_commanders;
mod average;
mod valid_lists;
mod id_win_rate;
mod deck_list;
mod ids_with_decklist;
mod all_cards;
mod card_list_map;
mod id_deck_list_map;

pub struct Store {
    commander_name: String,
    cache: Cache,
}

impl Store {
    pub fn new(commander_name: &String) -> Store {
        Store {
            commander_name: commander_name.clone(),
            cache: Cache::new(commander_name),
        }
    }
}

