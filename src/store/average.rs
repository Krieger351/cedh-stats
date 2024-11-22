use crate::data_types::win_rate::WinRate;
use crate::store::Store;
use anyhow::Result;

impl Store<'_> {
    pub async fn average(self: &Self) -> Result<WinRate> {
        let valid_ids = self.ids_with_deck_list().await?;
        let id_win_rate = self.deck_id_win_rate_map().await?;
        let mut acc = WinRate::from_f64(0.0);
        for id in valid_ids.iter() {
            acc = &acc + id_win_rate.get(id).unwrap();
        }
        acc = acc / valid_ids.len() as f64;
        Ok(acc)
    }
}