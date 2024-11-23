mod top_commanders;
mod average;
mod full_deck_id_win_rate_map;
pub mod deck_list;
mod all_ids_with_deck_list;
mod all_cards;
mod card_list_map;
mod top_decks;
mod all_commander_entries;
mod full_deck_id_deck_list_map;
mod all_deck_entries;

use crate::cache::{Cache, CommanderCache};
use crate::data_types::commander::Commander;
pub use top_decks::TopDeckMethod;

pub struct Store<'a> {
    commander: &'a Commander,
    cache: Cache,
    commander_cache: CommanderCache<'a>,
}

impl Store<'_> {
    pub async fn new(commander: &Commander) -> Store {
        Store {
            commander,
            cache: Cache::new(),
            commander_cache: CommanderCache::new(commander),
        }
    }

    pub fn commander(&self) -> &Commander {
        self.commander
    }
}

