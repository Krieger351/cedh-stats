mod cache;
mod commander_cache;

use anyhow::Result;
use de::Deserialize;
use ser::Serialize;
use serde::de;
use serde::ser;
use std::path::Path;

pub use cache::Cache;
pub use commander_cache::CommanderCache;
pub struct Key(str);

pub trait CacheController<'a> {
    fn key(&self, key: &str) -> String;
    fn full_key(&self, key: &str) -> String {
        format!(".cache/{}.json", self.key(key))
    }
    async fn read<T: for<'b> Deserialize<'b>>(&self, key: &str) -> Result<T> {
        let file_string = tokio::fs::read_to_string(self.full_key(key)).await?.to_string();

        let data = serde_json::from_str::<T>(&file_string)?;
        Ok(data)
    }

    async fn write<T: Serialize>(&self, key: &str, data: T) -> Result<()> {
        let key = self.full_key(key);
        let data_string = serde_json::to_string_pretty::<T>(&data)?;
        tokio::fs::create_dir_all(Path::new(&key).parent().unwrap()).await?;
        tokio::fs::write(key, data_string).await?;
        Ok(())
    }
}

pub trait Cacheable<'a, T: for<'b> Deserialize<'b> + Serialize>: Sized {
    type C<'c>: CacheController<'c>;
    async fn compute(&self) -> Result<T>;
    fn cache_file_path(&self) -> String;

    async fn load_or_compute(&self, cache: &Self::C<'a>) -> Result<T> {
        if let Ok(data) = cache.read::<T>(&self.cache_file_path()).await {
            return Ok(data);
        }

        let value = self.compute().await?;
        let _ = cache.write(&self.cache_file_path(), &value).await;

        Ok(value)
    }
}