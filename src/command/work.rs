use crate::data_structures::DeckId;
use crate::store::Store;

pub async fn exec(store: &Store) {
    let ids = store.deck_list(&DeckId::new("LS4Py5YSpU-ZcLyGqT5yJg".to_string())).await.unwrap();

    println!("{:#?}", ids)
}