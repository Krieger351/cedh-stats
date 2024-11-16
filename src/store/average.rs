use crate::store::Store;
use anyhow::Result;
impl Store {
    pub async fn average(self: &Self) -> Result<f32> {
        let valid_ids = self.ids_with_deck_list().await?;
        let id_win_rate = self.id_win_rate().await?;
        let mut acc = 0.0;
        for id in valid_ids.iter() {
            acc = acc + id_win_rate.get(id).unwrap();
        }
        acc = acc / valid_ids.len() as f32;
        Ok(acc)
    }
}