mod top_commanders;
mod average;
mod full_deck_id_win_rate_map;
pub mod deck_list;
mod all_ids_with_deck_list;
mod all_cards;
mod card_list_map;
mod top_decks;
pub mod all_commander_entries;
mod full_deck_id_deck_list_map;
mod all_deck_entries;
mod write_file;
mod commander_data;

use crate::cache::CommanderCache;
use crate::types::commander::Commander;
pub use top_decks::TopDeckMethod;

pub struct Store<'a> {
    commander: &'a Commander,
    cache: CommanderCache<'a>,
}

impl Store<'_> {
    pub fn new(commander: &Commander) -> Store {
        Store {
            commander,
            cache: CommanderCache::new(commander),
        }
    }

    pub fn commander(&self) -> &Commander {
        self.commander
    }
}

