mod store;
mod cache;
mod moxfield;
mod command;
mod data_types;
mod types;
mod data;
mod loader;

use anyhow::Result;
use clap::Parser;
use command::Command;
use dotenv::dotenv;


#[derive(Parser)]
#[command(version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let cli = Cli::parse();
    cli.command.exec().await
}

