use crate::store::Store;
use clap::Subcommand;

mod prefetch;
mod work;

#[derive(Subcommand, Debug)]
pub enum Command {
    Prefetch,
    Work,
}

impl Command {
    pub async fn exec(cmd: Command, store: &Store) {
        match cmd {
            Command::Prefetch => prefetch::exec(store).await,
            Command::Work => work::exec(store).await
        }
    }
}