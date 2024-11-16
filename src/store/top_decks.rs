use crate::data_structures::IdWinRate;
use crate::store::Store;
use anyhow::Result;
use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, ValueEnum)]
pub enum TopDeckMethod {
    Percent,
    Quartile,
    ZScore,
}

impl Display for TopDeckMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Store {
    pub async fn top_decks(&self, method: Option<TopDeckMethod>) -> Result<IdWinRate> {
        let method = method.unwrap_or(TopDeckMethod::Percent);
        let key = format!("meta/top_decks_{}", method).to_lowercase();

        if let Ok(data) = self.cache.read_commander(&key).await {
            Ok(data)
        } else {
            println!("here 1");
            let ids = self.id_win_rate().await?;

            let top_decks = match method {
                TopDeckMethod::Quartile => ids.into_top_decks_by_quartile(),
                TopDeckMethod::ZScore => ids.into_top_decks_by_z_score(),
                TopDeckMethod::Percent => ids.into_top_decks_by_percent(),
            };

            self.cache.write_commander(&key, &top_decks).await?;

            Ok(top_decks)
        }
    }
}