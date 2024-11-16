mod command;
mod store;
mod cache;
mod moxfield;
mod data_utils;
mod data_structures;
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
async fn main() {
    let args = Cli::parse();
    let store = Store::new(&args.commander);
    Command::exec(args.command, &store).await;
}

