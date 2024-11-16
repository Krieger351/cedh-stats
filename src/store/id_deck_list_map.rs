use crate::data_structures::IdDeckListMap;
use crate::store::Store;
use anyhow::Result;

impl Store {
    pub async fn id_deck_list_map(self: &Self) -> Result<IdDeckListMap> {
        if let Ok(data) = self.cache.read_commander("meta/id_deck_list_map").await {
            Ok(data)
        } else {
            let id_win_rate = self.id_win_rate().await?;
            let mut id_deck_list_map = IdDeckListMap::new();

            for id in id_win_rate.keys() {
                match self.deck_list(id).await? {
                    Some(list) => { id_deck_list_map.insert(id.clone(), list); }
                    _ => ()
                }
            }
            self.cache.write_commander("meta/id_deck_list_map", &id_deck_list_map).await?;
            Ok(id_deck_list_map)
        }
    }
}