use crate::cache::CacheController;
use crate::data_types::commander::Commander;

pub struct CommanderCache<'a>(&'a Commander);
impl CommanderCache<'_> {
    pub fn new(commander: &Commander) -> CommanderCache {
        CommanderCache(commander)
    }
}
impl CacheController<'_> for CommanderCache<'_> {
    fn key(&self, key: &str) -> String {
        format!("{}/{}", self.0, key)
    }
}