// use crate::cache::{Cacheable, CommanderCache, FileController};
// use crate::store::Store;
// use crate::types::deck_entry::DeckEntry;
// use crate::types::deck_entry_list::DeckEntryList;
// use anyhow::Result;
//
// pub struct FullDeckEntryListsReader();
//
// impl<'a> Cacheable<'a, DeckEntryList> for FullDeckEntryListsReader {
//     type C = CommanderCache<'a>;
//
//     async fn compute(&self) -> Result<DeckEntryList> {
//         panic!("Run prefetch first")
//     }
//
//     fn cache_file_path(&self) -> String {
//         "meta/all-deck-entries".to_string()
//     }
// }
//
// impl Store<'_> {
//     pub async fn write_all_deck_entries(&self, data: &[DeckEntry]) -> Result<()> {
//         self.cache().write(&FullDeckEntryListsReader().cache_file_path(), data).await
//     }
//     pub async fn all_deck_entries(self: &Self) -> Result<DeckEntryList> {
//         FullDeckEntryListsReader().load(self.cache()).await
//     }
// }
