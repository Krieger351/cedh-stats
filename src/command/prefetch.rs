use crate::store::Store;

pub async fn exec(store: &Store) {
    // let top_commanders = store.top_commanders().await.unwrap();
    // println!("{:?}", store.commander_entries_entries().await);
    store.commander_entries().await;

    // println!("{:?}", top_commanders);
}