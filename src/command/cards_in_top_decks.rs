use crate::data_structures::DeckList;
use crate::store::{Store, TopDeckMethod};
use anyhow::Result;

pub async fn exec(store: &Store, method: Option<TopDeckMethod>) -> Result<()> {
    let top_decks = store.top_decks(method).await?;
    let id_deck_list_map = store.id_deck_list_map().await?;

    let mut all_cards = DeckList::new();

    for deck_id in top_decks.keys() {
        if let Some(deck_list) = id_deck_list_map.get(deck_id) {
            all_cards.extend(deck_list.clone().into_iter());
        }
    }

    println!("The top decks for {} have {} unique cards", store.commander_name, &all_cards.len());

    for card in all_cards.iter() {
        println!("\t{}", card);
    }

    Ok(())
}