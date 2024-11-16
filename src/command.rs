use crate::store::{Store, TopDeckMethod};
use clap::Subcommand;

mod prefetch;
mod work;
mod win_rate_per_card;
mod cards_in_top_decks;

#[derive(Subcommand, Debug)]
pub enum Command {
    Prefetch,
    Work,
    WinRatePerCard,
    CardsInTopDecks { name: Option<TopDeckMethod> },
}

impl Command {
    pub async fn exec(cmd: Command, store: &Store) -> anyhow::Result<()> {
        match cmd {
            Command::Prefetch => prefetch::exec(store).await?,
            Command::Work => work::exec(store).await,
            Command::WinRatePerCard => win_rate_per_card::exec(store).await,
            Command::CardsInTopDecks { name } => cards_in_top_decks::exec(store, name).await?
        }
        Ok(())
    }
}