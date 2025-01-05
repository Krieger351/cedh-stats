// use crate::data_types::deck_id_win_rate_map::DeckIdWinRateMap;
use crate::Store;
use anyhow::Result;

struct FullDeckIdWinRateMapReader<'a>(&'a Store<'a>);

// impl<'a> Cacheable<'a, DeckIdWinRateMap> for FullDeckIdWinRateMapReader<'_> {
//     type C = CommanderCache<'a>;
//
//     async fn compute(&self) -> Result<DeckIdWinRateMap> {
//         todo!()
//         // Ok(self.0.all_commander_entries().await?.into_deck_entry_list().into_deck_id_win_rate_map())
//     }
//
//     fn cache_file_path(&self) -> String {
//         "meta/full-deck-id-win-rate-map".to_string()
//     }
// }

impl Store<'_> {
    pub async fn full_deck_id_win_rate_map(&self) -> Result<()> {
        todo!()
        // FullDeckIdWinRateMapReader(self).load_or_compute(&self.cache).await
    }
}