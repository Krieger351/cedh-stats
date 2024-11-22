mod store;
mod cache;
mod moxfield;
mod command;
mod data_types;

use crate::data_types::commander::Commander;
use anyhow::Result;
use clap::Parser;
use command::Command;
use store::Store;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[arg(value_parser = clap::value_parser!(Commander))]
    commander: Commander,
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let store = Store::new(&cli.commander).await;
    cli.command.exec(&store).await
}

