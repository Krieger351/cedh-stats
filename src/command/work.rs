use crate::store::{Store, TopDeckMethod};

pub async fn exec(store: &Store) {
    let ids = store.top_decks(Some(TopDeckMethod::Percent)).await.unwrap();
    println!("{:?}", ids)
}