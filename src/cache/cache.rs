use crate::cache::CacheController;

pub struct Cache();

impl Cache {
    pub fn new() -> Cache {
        Cache()
    }
}

impl CacheController<'_> for Cache {
    fn key(&self, key: &str) -> String {
        key.to_string()
    }
}