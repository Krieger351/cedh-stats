use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::deck_entry_list::DeckEntryList;
use crate::store::Store;
use anyhow::Result;
use reqwest;
use serde;

struct FullDeckEntryListsReader<'a>(&'a Store<'a>);

impl Cacheable<'_, DeckEntryList> for FullDeckEntryListsReader<'_> {
    type C<'a> = CommanderCache<'a>;
    async fn compute(&self) -> Result<DeckEntryList> {
        println!("{:?}", self.0.all_commander_entries().await?);
        Ok(self.0.all_commander_entries().await?.into_deck_entry_list())
    }

    fn cache_file_path(&self) -> String {
        "meta/full-deck-entry-list".to_string()
    }
}
impl Store<'_> {
    pub async fn full_deck_entry_list(self: &Self) -> Result<DeckEntryList> {
        FullDeckEntryListsReader(self).load_or_compute(&self.commander_cache).await
    }
}
