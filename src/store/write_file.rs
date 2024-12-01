use crate::cache::CacheController;
use crate::store::Store;
use serde::Serialize;

impl Store<'_> {
    pub(crate) async fn write_file<T: Serialize>(&self, key: &str, data: T) -> anyhow::Result<()> {
        self.cache.write(key, data).await
    }
}