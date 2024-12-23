mod store;
mod cache;
mod command;
// mod data_types;
mod types;
mod remote_data;

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

