use crate::cache::{Cacheable, CommanderCache};
use crate::data::edh_top_sixteen::CommanderEntries;
use crate::store::Store;
use crate::types::commander::Commander;
use anyhow::Result;
use reqwest;

pub struct AllCommanderEntriesReader<'a>(&'a Commander);

impl<'a> Cacheable<'a, CommanderEntries> for AllCommanderEntriesReader<'_> {
    type C = CommanderCache<'a>;

    async fn compute(&self) -> Result<CommanderEntries> {
        Ok(CommanderEntries(Vec::new()))
        // EdhTopSixteen::get_entries(self.0).await
    }

    fn cache_file_path(&self) -> String {
        "commander-entries".to_string()
    }
}

impl Store<'_> {
    pub async fn all_commander_entries(self: &Self) -> Result<CommanderEntries> {
        AllCommanderEntriesReader(self.commander).load_or_compute(&self.cache).await
    }
}
