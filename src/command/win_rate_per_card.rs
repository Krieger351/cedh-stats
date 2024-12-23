use crate::command::Executor;
use crate::store::Store;
use crate::types::deck_data_list::TopDeckMethod;
use std::collections::{HashMap, HashSet};

pub struct WinRatePerCard {
    method: Vec<TopDeckMethod>,
}
impl WinRatePerCard {
    pub fn new(method: Option<Vec<TopDeckMethod>>) -> Self {
        Self { method: method.unwrap_or(vec![]) }
    }
}
impl Executor for WinRatePerCard {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let entries = store.all_decks().await?.into_top_decks_with_methods(&self.method[..]);
        let mut map = HashMap::new();
 
        for entry in entries.iter() {
            let card_list = store.deck_list(entry.id()).await?.into_iter();
            for card in card_list {
                map.entry(card).or_insert(HashSet::new()).insert(entry.id());
            }
        }

        for (card, set) in map {
            println!("{}: {}", card, set.len())
        }

        Ok(())
    }
}