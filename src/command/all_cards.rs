use crate::command::Executor;
use crate::store::Store;

pub struct AllCards();


impl Executor for AllCards {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let all_cards = store.all_cards().await?;

        println!("There are {} unique cards.", all_cards.len());
        for card in all_cards.iter() {
            println!("{}", card);
        }
        Ok(())
    }
}