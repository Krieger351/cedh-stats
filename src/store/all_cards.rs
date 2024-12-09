use crate::cache::{Cacheable, CommanderCache};
use crate::store::Store;
use crate::types::card_set::CardSet;
use anyhow::Result;

struct AllCardsReader<'a>(&'a Store<'a>);

impl<'a> Cacheable<'a, CardSet> for AllCardsReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<CardSet> {
        let entries = self.0.all_commander_entries().await?;
        let mut all_cards = CardSet::new();
        for entry in entries.iter() {
            if let Some(id) = entry.get_id() {
                if let Some(list) = self.0.deck_list(&id).await? {
                    all_cards.extend(list);
                }
            }
        }
        Ok(all_cards)
    }

    fn cache_file_path(&self) -> String {
        "meta/all_cards".into()
    }
}

impl Store<'_> {
    pub async fn all_cards(&self) -> Result<CardSet> {
        AllCardsReader(self).load_or_compute(&self.cache).await
    }
}