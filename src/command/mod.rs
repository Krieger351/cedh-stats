use crate::command::cards_in_top_decks::CardsInTopDecks;
use crate::command::prefetch::Prefetch;
use crate::command::win_rate_per_card::WinRatePerCard;
use crate::command::work::Work;
use crate::store::Store;
use crate::store::TopDeckMethod;
use clap::Subcommand;

mod prefetch;
mod work;
mod win_rate_per_card;
mod cards_in_top_decks;

pub trait Executor {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Prefetch,
    Work,
    WinRatePerCard,
    CardsInTopDecks { method: Option<TopDeckMethod> },
}
impl Command {
    pub async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        match self {
            Command::Prefetch => run_command(store, Prefetch {}).await,
            Command::Work => run_command(store, Work {}).await,
            Command::WinRatePerCard => run_command(store, WinRatePerCard {}).await,
            Command::CardsInTopDecks { method } => run_command(store, CardsInTopDecks::new(method)).await,
        }
    }
}
async fn run_command(store: &Store<'_>, command: impl Executor) -> anyhow::Result<()> {
    command.exec(store).await
}