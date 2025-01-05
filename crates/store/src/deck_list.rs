use crate::Store;
use anyhow::{anyhow, Result};
use cache::{Cacheable, CommanderCache};
use types::card_list::CardList;
use types::deck_id::DeckId;

pub struct DeckListReader<'a>(&'a Store<'a>, &'a DeckId);
impl DeckListReader<'_> {
    pub fn new<'a>(store: &'a Store, deck_id: &'a DeckId) -> DeckListReader<'a> {
        DeckListReader(store, deck_id)
    }
}


impl<'a> Cacheable<'a, CardList> for DeckListReader<'a> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<CardList> {
        Ok(self.0.all_decks().await?.iter().find(|x| x.id() == self.1).ok_or(anyhow!("Deck Id Missing"))?.list().clone())
    }

    fn cache_file_path(&self) -> String {
        format!("deck-list/{}", self.1)
    }
}
impl Store<'_> {
    pub async fn deck_list(self: &Self, id: &DeckId) -> Result<CardList> {
        DeckListReader(self, id).load_or_compute(&self.cache).await
    }
}