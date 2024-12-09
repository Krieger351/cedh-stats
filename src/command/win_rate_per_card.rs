use crate::command::Executor;
use crate::store::{Store, TopDeckMethod};
use crate::types::win_rate::WinRate;

pub struct WinRatePerCard {}

impl Executor for WinRatePerCard {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let top_lists = store.top_decks(&TopDeckMethod::default()).await?;
        let card_list_id_map = store.card_deck_id_set_map().await?;

        for (card, list) in card_list_id_map.iter() {
            let included_win_rate = list.iter().filter_map(|x| top_lists.get(x)).collect::<Vec<&WinRate>>();
            let len = included_win_rate.len();
            if len > 5 {
                println!("{}: {}", card, included_win_rate.into_iter().sum::<WinRate>() / len as f64);
            }
        }
        Ok(())
    }
}