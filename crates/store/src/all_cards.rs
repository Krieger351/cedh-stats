use crate::Store;
use anyhow::Result;
use cache::{Cacheable, CommanderCache};
use types::card_set::CardSet;

struct AllCardsReader<'a>(&'a Store<'a>);

impl<'a> Cacheable<'a, CardSet> for AllCardsReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<CardSet> {
        let decks = self.0.all_decks().await?;
        Ok(decks.all_cards())
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