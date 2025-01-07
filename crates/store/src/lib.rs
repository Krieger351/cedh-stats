mod top_commanders;
mod average;
pub mod deck_list;
mod all_cards;
mod top_decks;
mod all_deck_entries;
mod write_file;
mod all_decks;
mod clusters;

use cache::CommanderCache;
use types::commander::Commander;

pub struct Store<'a> {
    commander: &'a Commander,
    cache: CommanderCache<'a>,
}

impl Store<'_> {
    pub fn cache(&self) -> &CommanderCache<'_> {
        &self.cache
    }
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

