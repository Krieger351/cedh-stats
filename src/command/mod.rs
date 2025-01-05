use crate::command::all_cards::AllCards;
use crate::command::cards_in_top_decks::CardsInTopDecks;
use crate::command::find_clusters::FindClusters;
use crate::command::prefetch::Prefetch;
use crate::command::search_decks_for_card::SearchDecksForCard;
use crate::command::setup_env::SetupEnv;
use crate::command::win_rate_per_card::WinRatePerCard;
use clap::Subcommand;
use std::str::FromStr;
use store::Store;
use types::card::Card;
use types::commander::Commander;
use types::deck_data_list::TopDeckMethod;
use types::similarity_score::SimilarityScore;

mod prefetch;
mod work;
mod win_rate_per_card;
mod cards_in_top_decks;
mod all_cards;
mod setup_env;
mod find_clusters;
mod search_decks_for_card;

pub trait Executor {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Prefetch {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
    },
    SetupEnv {
        #[arg(value_parser = clap::value_parser!(Commander))]
        commander: Option<Commander>,
    },
    AllCards {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
    },
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
    FindClusters {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        method: Option<Vec<TopDeckMethod>>,
        #[clap(short, long, value_parser)]
        card_to_look_for: Option<Card>,
        #[clap(short, long, value_parser)]
        threshold: Option<SimilarityScore>,
    },
    SearchDecksForCard {
        #[arg(value_parser = clap::value_parser!(Commander), env = "COMMANDER")]
        commander: Commander,
        #[clap(short, long, value_parser, num_args = 1.., value_delimiter = ' ')]
        method: Option<Vec<TopDeckMethod>>,
        #[clap(short, long, value_parser)]
        card: Option<Card>,
    },
}
impl Command {
    pub async fn exec(self) -> anyhow::Result<()> {
        match self {
            Command::Prefetch { commander } => {
                run_command(&Store::new(&commander), Prefetch {}).await
            }
            Command::SetupEnv { commander } => run_command(&Store::new(&Commander::from_str("")?), SetupEnv::new(commander)).await,
            Command::AllCards { commander } => run_command(&Store::new(&commander), AllCards()).await,
            Command::WinRatePerCard { commander, method } => run_command(&Store::new(&commander), WinRatePerCard::new(method)).await,
            Command::CardsInTopDecks { commander, method } => run_command(&Store::new(&commander), CardsInTopDecks::new(method)).await,
            Command::FindClusters { commander, method, card_to_look_for, threshold } => run_command(&Store::new(&commander), FindClusters::new(method, card_to_look_for, threshold)).await,
            Command::SearchDecksForCard { commander, method, card } => run_command(&Store::new(&commander), SearchDecksForCard::new(card, method)).await,
        }
    }
}
async fn run_command(store: &Store<'_>, command: impl Executor) -> anyhow::Result<()> {
    command.exec(store).await
}