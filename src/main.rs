mod command;
mod store;
mod cache;

use command::Command;

use clap::Parser;
use crate::store::Store;

#[derive(Parser)]
#[command(version)]
struct Cli {
    commander: String,
    #[clap(subcommand)]
    command: Command
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let store = Store::new(&args.commander);
    Command::exec(args.command, &store).await;
}
