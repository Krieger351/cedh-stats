use crate::command::all_cards::AllCards;
use crate::command::cards_in_top_decks::CardsInTopDecks;
use crate::command::prefetch::Prefetch;
use crate::command::win_rate_per_card::WinRatePerCard;
use crate::command::wip::Wip;
use crate::command::work::Work;
use crate::store::Store;
use crate::types::commander::Commander;
use clap::Subcommand;
use std::str::FromStr;

mod prefetch;
mod work;
mod win_rate_per_card;
mod cards_in_top_decks;
mod all_cards;
mod wip;

pub trait Executor {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Prefetch {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
    },
    Wip {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
    },
    // Work,
    // WinRatePerCard,
    // CardsInTopDecks { method: Option<TopDeckMethod> },
    // AllCards,
}
impl Command {
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            Command::Prefetch { commander } => {
                run_command(&Store::new(&commander), Prefetch {}).await
            }
            Command::Wip { commander } => run_command(&Store::new(&commander), Wip()).await
            // Command::Work => run_command(store, Workork {}).await,
            // Command::WinRatePerCard => run_command(store, WinRatePerCard {}).await,
            // Command::CardsInTopDecks { method } => run_command(store, CardsInTopDecks::new(method)).await,
            // Command::AllCards => run_command(store, AllCards {}).await,
        }
    }
}
async fn run_command(store: &Store<'_>, command: impl Executor) -> anyhow::Result<()> {
    command.exec(store).await
}