use crate::command::Executor;
use crate::store::{Store, TopDeckMethod};
use crate::types::card_set::CardSet;
use anyhow::Result;

pub struct CardsInTopDecks {
    method: TopDeckMethod,
}
impl CardsInTopDecks {
    pub fn new(method: Option<TopDeckMethod>) -> Self {
        Self { method: method.unwrap_or_default() }
    }
}
impl Executor for CardsInTopDecks {
    async fn exec(&self, store: &Store<'_>) -> Result<()> {
        let top_decks = store.top_decks(&self.method).await?;
        let id_deck_list_map = store.full_deck_id_deck_list_map().await?;

        let mut all_cards = CardSet::new();

        for deck_id in top_decks.keys() {
            if let Some(deck_list) = id_deck_list_map.get(deck_id) {
                all_cards.extend(deck_list.clone().into_iter());
            }
        }

        println!("The top decks for {} have {} unique cards", store.commander(), &all_cards.len());

        for card in all_cards.iter() {
            println!("\t{}", card);
        }

        Ok(())
    }
}