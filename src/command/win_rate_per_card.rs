use crate::command::Executor;
use store::Store;
use types::deck_data_list::TopDeckMethod;

pub struct WinRatePerCard {
    method: Vec<TopDeckMethod>,
}
impl WinRatePerCard {
    pub fn new(method: Option<Vec<TopDeckMethod>>) -> Self {
        Self { method: method.unwrap_or(vec![]) }
    }
}
impl Executor for WinRatePerCard {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        let entries = store.all_decks().await?.into_top_decks_with_methods(&self.method[..]);
        println!("Average win rate: {:2}", store.all_decks().await?.average());

        let mut averages = entries.win_rate_per_card().into_iter().collect::<Vec<_>>();
        averages.sort();

        for (card, average) in averages {
            println!("{card:width$.20}  {average:.5}", width = 20);
        }

        Ok(())
    }
}