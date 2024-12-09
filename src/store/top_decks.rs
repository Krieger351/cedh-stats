use crate::cache::{Cacheable, CommanderCache};
use crate::data_types::deck_id_win_rate_map::DeckIdWinRateMap;
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

impl Default for TopDeckMethod {
    fn default() -> Self {
        Self::Percent
    }
}

impl Display for TopDeckMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

struct TopDeckReader<'a>(&'a Store<'a>, &'a TopDeckMethod);

impl<'a> Cacheable<'a, DeckIdWinRateMap> for TopDeckReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<DeckIdWinRateMap> {
        let ids = self.0.full_deck_id_win_rate_map().await?;

        Ok(match self.1 {
            TopDeckMethod::Quartile => ids.into_top_decks_by_quartile(),
            TopDeckMethod::ZScore => ids.into_top_decks_by_z_score(),
            TopDeckMethod::Percent => ids.into_top_decks_by_percent(),
        })
    }

    fn cache_file_path(&self) -> String {
        format!("meta/top_decks_{}", self.1).to_lowercase()
    }
}

impl Store<'_> {
    pub async fn top_decks(&self, method: &TopDeckMethod) -> Result<DeckIdWinRateMap> {
        TopDeckReader(self, method).load_or_compute(&self.cache).await
    }
}