use crate::store::Store;
use crate::types::win_rate::WinRate;
use anyhow::Result;

impl Store<'_> {
    pub async fn average(self: &Self) -> Result<WinRate> {
        todo!()
        // let valid_ids = self.all_ids_with_deck_list().await?;
        // let id_win_rate = self.full_deck_id_win_rate_map().await?;
        // let mut acc = WinRate::MIN;
        // for id in valid_ids.iter() {
        //     acc = &acc + id_win_rate.get(id).unwrap();
        // }
        // acc = acc / valid_ids.len() as f64;
        // Ok(acc)
    }
}