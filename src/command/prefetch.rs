use crate::command::Executor;
use crate::store::Store;


pub struct Prefetch {}

impl Executor for Prefetch {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let _ = store.top_commanders().await?;
        let id_win_rate = store.full_deck_id_win_rate_map().await?;
        for id in id_win_rate.keys() {
            println!("Reading: {}", id);
            match store.deck_list(id).await {
                Ok(_) => {
                    println!("\tSuccess");
                }
                Err(_) => {
                    println!("\tFailure");
                }
            }
        }
        Ok(())
    }
}
