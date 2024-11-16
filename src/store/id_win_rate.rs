use crate::data_structures::IdWinRate;
use crate::store::Store;
use anyhow::Result;

impl Store {
    pub async fn id_win_rate(self: &Self) -> Result<IdWinRate> {
        if let Ok(data) = self.cache.read_commander("meta/id_win_rate").await {
            Ok(data)
        } else {
            let entries = self.commander_entries().await?;
            let mut data = IdWinRate::new();

            for entry in entries.iter() {
                if entry.is_valid() {
                    data.insert(entry.get_id().unwrap(), entry.win_rate.clone().unwrap());
                }
            }
            self.cache.write_commander("meta/id_win_rate", &data).await?;
            Ok(data)
        }
    }
}