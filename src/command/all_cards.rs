use crate::command::Executor;
use crate::store::Store;

pub struct AllCards {}


impl Executor for AllCards {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let all_cards = store.all_cards().await?;

        let full_deck_id_deck_list_map = store.full_deck_id_deck_list_map().await?;

        println!("There are {} decks.", full_deck_id_deck_list_map.len());
        for (id, list) in full_deck_id_deck_list_map.iter() {
            println!("{}: {}", id, list.len());
        }


        println!("There are {} unique cards.", all_cards.len());
        for card in all_cards.iter() {
            println!("{}", card);
        }
        Ok(())
    }
}