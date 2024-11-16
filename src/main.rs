mod command;
mod store;
mod cache;
mod moxfield;
mod data_structures;
use crate::data_structures::Commander;
use anyhow::Result;
use clap::Parser;
use command::Command;
use store::Store;

#[derive(Parser)]
#[command(version)]
struct Cli {
    commander: String,
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    if let Ok(store) = Store::new(&Commander::from_string(args.commander)).await {
        Command::exec(args.command, &store).await?;
    }
    Ok(())
}

