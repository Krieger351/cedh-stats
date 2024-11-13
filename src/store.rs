use crate::cache::Cache;

mod commander_entries;
mod top_commanders;

pub struct Store {
    commander_name: String,
    cache: Cache,
}

impl Store {
    pub fn new(commander_name: &String) -> Store {
        Store {
            commander_name: commander_name.clone(),
            cache: Cache::new(commander_name),
        }
    }
}

