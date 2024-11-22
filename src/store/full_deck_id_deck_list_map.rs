use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::deck_id_deck_list_map::DeckIdDeckListMap;
use crate::store::Store;
use anyhow::Result;

struct FullDeckIdDeckListMapReader<'a>(&'a Store<'a>);

impl Cacheable<'_, DeckIdDeckListMap> for FullDeckIdDeckListMapReader<'_> {
    type C<'c> = CommanderCache<'c>;

    async fn compute(&self) -> Result<DeckIdDeckListMap> {
        let id_win_rate = self.0.deck_id_win_rate_map().await?;
        let mut id_deck_list_map = DeckIdDeckListMap::new();

        for id in id_win_rate.keys() {
            match self.0.deck_list(id).await? {
                Some(list) => { id_deck_list_map.insert(id.clone(), list); }
                _ => ()
            }
        }
        Ok(id_deck_list_map)
    }

    fn cache_file_path(&self) -> String {
        "meta/deck_id_deck_list_map".to_string()
    }
}

impl Store<'_> {
    pub async fn full_deck_id_deck_list_map(self: &Self) -> Result<DeckIdDeckListMap> {
        FullDeckIdDeckListMapReader(self).load_or_compute(&self.commander_cache).await
    }
}