use crate::cache::{Cacheable, CommanderCache};
use crate::store::Store;
use crate::types::card_set::CardSet;
use anyhow::Result;

struct AllCardsReader<'a>(&'a Store<'a>);

impl<'a> Cacheable<'a, CardSet> for AllCardsReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<CardSet> {
        let decks = self.0.all_decks().await?;
        let mut all_cards = CardSet::new();
        decks.into_iter().for_each(|b| all_cards.extend(b.list().iter().cloned()));
        Ok(all_cards)
    }

    fn cache_file_path(&self) -> String {
        "meta/all-cards".into()
    }
}

impl Store<'_> {
    pub async fn all_cards(&self) -> Result<CardSet> {
        AllCardsReader(self).load_or_compute(&self.cache).await
    }
}