use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::card_deck_id_set_map::CardDeckIdSetMap;
use crate::data_types::deck_id_set::DeckIdSet;
use crate::store::Store;
use anyhow::Result;

struct CardDeckIdSetMapReader<'a>(&'a Store<'a>);

impl Cacheable<'_, CardDeckIdSetMap> for CardDeckIdSetMapReader<'_> {
    type C<'a> = CommanderCache<'a>;
    async fn compute(&self) -> Result<CardDeckIdSetMap> {
        let mut card_list_map = CardDeckIdSetMap::new();
        let ids = self.0.full_deck_id_deck_list_map().await?;

        for (id, list) in ids.iter() {
            for card in list.iter() {
                match card_list_map.get_mut(card) {
                    None => {
                        let mut ids = DeckIdSet::new();
                        ids.insert(id.clone());
                        card_list_map.insert(card.clone(), ids);
                    }
                    Some(ids) => {
                        ids.insert(id.clone());
                    }
                }
            }
        }
        Ok(card_list_map)
    }

    fn cache_file_path(&self) -> String {
        "meta/card_list_id_map".into()
    }
}

impl Store<'_> {
    pub async fn card_deck_id_set_map(self: &Self) -> Result<CardDeckIdSetMap> {
        CardDeckIdSetMapReader(self).load_or_compute(&&self.commander_cache).await
    }
}