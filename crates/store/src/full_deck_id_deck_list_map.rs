// use crate::data_types::deck_id_deck_list_map::DeckIdDeckListMap;
use crate::Store;
use anyhow::Result;

struct FullDeckIdDeckListMapReader<'a>(&'a Store<'a>);

// impl<'a> Cacheable<'a, DeckIdDeckListMap> for FullDeckIdDeckListMapReader<'_> {
//     type C = CommanderCache<'a>;
//
//     async fn compute(&self) -> Result<DeckIdDeckListMap> {
//         todo!();
//         // let full_deck_entry_list = self.0.full_deck_entry_list().await?;
//         //
//         // let mut map = DeckIdDeckListMap::new();
//         //
//         // for entry in full_deck_entry_list.into_iter() {
//         //     if let Some(list) = self.0.deck_list(entry.id()).await? {
//         //         map.insert(entry.id().clone(), list);
//         //     }
//         // }
//         // Ok(map)
//     }
//
//     fn cache_file_path(&self) -> String {
//         "meta/full-deck-id-deck-list-map".to_string()
//     }
// }

impl Store<'_> {
    pub async fn full_deck_id_deck_list_map(self: &Self) -> Result<()> {
        todo!()
        // FullDeckIdDeckListMapReader(self).load_or_compute(&self.cache).await
    }
}