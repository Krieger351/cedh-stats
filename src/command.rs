use crate::store::Store;

mod prefetch;

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Prefetch
}

impl Command {
    pub async fn exec(cmd: Command, store: &Store) {
        match cmd { Command::Prefetch => prefetch::exec(store).await }
    }
}