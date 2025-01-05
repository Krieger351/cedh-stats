use crate::Store;
use anyhow::Result;
use cache::{Cacheable, CommanderCache};
use types::deck_data_list::DeckDataList;
use types::deck_data_list::TopDeckMethod;

struct TopDeckReader<'a>(&'a Store<'a>, &'a TopDeckMethod);

impl<'a> Cacheable<'a, DeckDataList> for TopDeckReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<DeckDataList> {
        let ids = self.0.all_decks().await?;

        Ok(ids.into_top_decks(self.1))
    }

    fn cache_file_path(&self) -> String {
        format!("meta/top_decks_{}", self.1).to_lowercase()
    }
}

impl Store<'_> {
    pub async fn top_decks(&self, method: &TopDeckMethod) -> Result<DeckDataList> {
        TopDeckReader(self, method).load_or_compute(&self.cache).await
    }
}