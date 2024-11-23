use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::deck_id_set::DeckIdSet;
use crate::store::Store;
use anyhow::Result;

struct AllIdsWithDeckListReader<'a>(&'a Store<'a>);

impl Cacheable<'_, DeckIdSet> for AllIdsWithDeckListReader<'_> {
    type C<'c> = CommanderCache<'c>;

    async fn compute(&self) -> Result<DeckIdSet> {
        let id_deck_list_map = self.0.full_deck_id_deck_list_map().await?;
        Ok(id_deck_list_map.keys().cloned().collect())
    }

    fn cache_file_path(&self) -> String {
        "meta/all_ids_with_deck_list".to_string()
    }
}

impl Store<'_> {
    pub async fn all_ids_with_deck_list(self: &Self) -> Result<DeckIdSet> {
        AllIdsWithDeckListReader(self).load_or_compute(&self.commander_cache).await
    }
}