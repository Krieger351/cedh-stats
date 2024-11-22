use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::deck_id::DeckId;
use crate::data_types::deck_list::DeckList;
use crate::moxfield::Moxfield;
use crate::store::Store;
use anyhow::Result;

pub struct DeckListReader<'a>(&'a DeckId);
impl DeckListReader<'_> {
    pub fn new(id: &DeckId) -> DeckListReader {
        DeckListReader(id)
    }
}

impl Cacheable<'_, Option<DeckList>> for DeckListReader<'_> {
    type C<'a> = CommanderCache<'a>;
    async fn compute(&self) -> Result<Option<DeckList>> {
        Ok(Moxfield::get_list(self.0).await.unwrap_or(None))
    }

    fn cache_file_path(&self) -> String {
        format!("deck_list/{}", self.0)
    }
}

impl Store<'_> {
    pub async fn deck_list(self: &Self, id: &DeckId) -> Result<Option<DeckList>> {
        DeckListReader(id).load_or_compute(&&self.commander_cache).await
    }
}