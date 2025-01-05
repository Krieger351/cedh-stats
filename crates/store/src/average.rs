use crate::Store;
use anyhow::Result;
use types::deck_id_set::DeckIdSet;
use types::win_rate::WinRate;

impl Store<'_> {
    pub async fn average(&self, sets: &DeckIdSet) -> Result<WinRate> {
        Ok(self.all_decks().await?.pick_decks(sets).average())
    }
}