use crate::cache::{Cacheable, CommanderCache};
use crate::remote_data::edh_top_sixteen;
use crate::store::Store;
use crate::types::deck_data_list::DeckDataList;
use anyhow::Result;

struct AllDecksReader<'a>(&'a Store<'a>);

impl<'a> Cacheable<'a, DeckDataList> for AllDecksReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<DeckDataList> {
        edh_top_sixteen::get_entries(self.0.commander).await
    }

    fn cache_file_path(&self) -> String {
        "all-decks".into()
    }
}

impl Store<'_> {
    pub async fn fetch_all_decks(&self) -> Result<DeckDataList> {
        AllDecksReader(self).load_or_compute(&self.cache).await
    }
    pub async fn all_decks(&self) -> Result<DeckDataList> {
        AllDecksReader(self).load(&self.cache).await
    }
}