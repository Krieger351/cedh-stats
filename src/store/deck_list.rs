use crate::cache::CommanderCache;
use crate::types::card_list::CardList;
use crate::{
    cache::Cacheable,
    moxfield::Moxfield,
    store::Store,
    types::deck_id::DeckId,
};
use anyhow::Result;

pub struct DeckListReader<'a>(&'a DeckId);
impl DeckListReader<'_> {
    pub fn new(id: &DeckId) -> DeckListReader {
        DeckListReader(id)
    }
}

impl<'a> Cacheable<'a, Option<CardList>> for DeckListReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<Option<CardList>> {
        Ok(Moxfield::get_list(self.0).await.unwrap_or(None))
    }

    fn cache_file_path(&self) -> String {
        format!("deck_list/{}", self.0)
    }
}

impl Store<'_> {
    pub async fn deck_list(self: &Self, id: &DeckId) -> Result<Option<CardList>> {
        DeckListReader(id).load_or_compute(&self.cache).await
    }
}