use crate::command::Executor;
use anyhow::Result;
use store::Store;
use types::card_set::CardSet;
use types::deck_data_list::TopDeckMethod;

pub struct CardsInTopDecks {
    method: Vec<TopDeckMethod>,
}
impl CardsInTopDecks {
    pub fn new(method: Option<Vec<TopDeckMethod>>) -> Self {
        Self { method: method.unwrap_or(vec![TopDeckMethod::Standing]) }
    }
}

impl Executor for CardsInTopDecks {
    async fn exec(self, store: &Store<'_>) -> Result<()> {
        let entries = store.all_decks().await?.into_top_decks_with_methods(&self.method[..]);

        let mut card_set = CardSet::new();

        for entry in entries.iter() {
            card_set.extend(store.deck_list(entry.id()).await?);
        }

        println!("There are {} top decks with {} unique cards for the sequence: {}", entries.len(), card_set.len(), &self.method.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
        println!();
        println!();
        let mut cards = card_set.iter().collect::<Vec<_>>();
        cards.sort();
        let max_width: usize = cards.iter().map(|s| s.to_string().len()).max().unwrap_or(0) + 2;
        for (index, card) in cards.iter().enumerate() {
            print!("{card:<width$}", width = max_width);
            if index % 3 == 2 {
                println!();
            }
        }

        Ok(())
    }
}