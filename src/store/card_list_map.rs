use crate::data_structures::{CardListIdMap, DeckId};
use crate::store::Store;
use anyhow::Result;
use std::collections::HashSet;

impl Store {
    pub async fn card_list_map(self: &Self) -> Result<CardListIdMap> {
        if let Ok(data) = self.cache.read_commander("meta/card_list_map").await {
            Ok(data)
        } else {
            let all_cards = self.all_cards().await?;

            let mut card_list_map = CardListIdMap::new();
            let ids = self.id_deck_list_map().await?;

            for (id, list) in ids.iter() {
                for card in list.iter() {
                    match card_list_map.get_mut(card) {
                        None => {
                            let mut ids = HashSet::<DeckId>::new();
                            ids.insert(id.clone());
                            card_list_map.insert(card.clone(), ids);
                        }
                        Some(ids) => {
                            ids.insert(id.clone());
                        }
                    }
                }
            }

            self.cache.write_commander("meta/card_list_map", &card_list_map).await?;
            Ok(card_list_map)
        }
    }
}