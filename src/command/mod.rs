use crate::command::all_cards::AllCards;
use crate::command::cards_in_top_decks::CardsInTopDecks;
use crate::command::prefetch::Prefetch;
use crate::command::setup_env::SetupEnv;
use crate::command::win_rate_per_card::WinRatePerCard;
use crate::command::work::Work;
use crate::store::Store;
use crate::types::commander::Commander;
use crate::types::deck_data_list::TopDeckMethod;
use clap::Subcommand;
use std::str::FromStr;

mod prefetch;
mod work;
mod win_rate_per_card;
mod cards_in_top_decks;
mod all_cards;
mod setup_env;

pub trait Executor {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Prefetch {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
    },
    SetupEnv,
    AllCards {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
    },
    // Work,
    WinRatePerCard {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        method: Option<Vec<TopDeckMethod>>,
    },
    CardsInTopDecks {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        method: Option<Vec<TopDeckMethod>>,
    },
    // AllCards,
}
impl Command {
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            Command::Prefetch { commander } => {
                run_command(&Store::new(&commander), Prefetch {}).await
            }
            Command::SetupEnv => run_command(&Store::new(&Commander::from_str("")?), SetupEnv()).await,
            Command::AllCards { commander } => run_command(&Store::new(&commander), AllCards()).await,
            Command::WinRatePerCard { commander, method } => run_command(&Store::new(&commander), WinRatePerCard::new(method)).await,
            Command::CardsInTopDecks { commander, method } => run_command(&Store::new(&commander), CardsInTopDecks::new(method)).await,
        }
    }
}
async fn run_command(store: &Store<'_>, command: impl Executor) -> anyhow::Result<()> {
    command.exec(store).await
}