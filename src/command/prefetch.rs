use crate::store::Store;

pub async fn exec(store: &Store) {
    let id_win_rate = store.id_win_rate().await.unwrap();
    for id in id_win_rate.keys() {
        println!("Reading: {}", id);
        match store.deck_list(&id).await {
            Ok(_) => {
                println!("\tSuccess");
            }
            Err(_) => {
                println!("\tFailure");
            }
        }
    }
}
