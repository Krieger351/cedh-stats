use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::deck_list::DeckList;
use crate::store::Store;
use anyhow::Result;

struct AllCardsReader<'a>(&'a Store<'a>);

impl Cacheable<'_, DeckList> for AllCardsReader<'_> {
    type C<'a> = CommanderCache<'a>;
    async fn compute(&self) -> Result<DeckList> {
        let entries = self.0.all_commander_entries().await?;
        let mut all_cards = DeckList::new();
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
    pub async fn all_cards(&self) -> Result<DeckList> {
        AllCardsReader(self).load_or_compute(&&self.commander_cache).await
    }
}