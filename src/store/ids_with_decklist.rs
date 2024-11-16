use crate::data_structures::DeckId;
use crate::store::Store;
use anyhow::Result;
use std::collections::HashSet;

impl Store {
    pub async fn ids_with_deck_list(self: &Self) -> Result<HashSet<DeckId>> {
        if let Ok(data) = self.cache.read_commander("meta/ids_with_deck_list").await {
            Ok(data)
        } else {
            let id_deck_list_map = self.id_deck_list_map().await?;
            let ids_with_deck_list: HashSet<DeckId> = id_deck_list_map.keys().cloned().collect();
            self.cache.write_commander("meta/ids_with_deck_list", &ids_with_deck_list).await?;
            Ok(ids_with_deck_list)
        }
    }
}