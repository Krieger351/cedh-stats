extern crate proc_macro;
use crate::types::commander::Commander;
use de::Deserialize;
use ser::Serialize;
use serde::de;
use serde::ser;
use std::path::Path;

pub trait FileController {
    fn file_safe_string(str: &str) -> String {
        format!("{}", str.to_lowercase().replace(&['(', ')', ',', '\"', ';', ':', '\'', '/', '\\'][..], "").replace(" ", "_"))
    }
    fn key(&self, key: &str) -> String;
    async fn read<T: for<'b> Deserialize<'b>>(&self, key: &str) -> anyhow::Result<T> {
        let safe_key = &self.key(key);
        let file_string = tokio::fs::read_to_string(safe_key).await?.to_string();
        let data = serde_json::from_str::<T>(&file_string)?;
        Ok(data)
    }

    async fn write<T: Serialize>(&self, key: &str, data: T) -> anyhow::Result<()> {
        let safe_key = &self.key(key);
        let data_string = serde_json::to_string_pretty::<T>(&data)?;
        tokio::fs::create_dir_all(Path::new(&safe_key).parent().unwrap()).await?;
        tokio::fs::write(safe_key, data_string).await?;
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Cache();

impl Cache {
    pub fn new() -> Self {
        Self()
    }
}
impl FileController for Cache {
    fn key(&self, key: &str) -> String {
        format!(".cache/{key}.json")
    }
}

#[derive(Clone, Debug)]
pub struct CommanderCache<'a> {
    commander: &'a Commander,
}

impl<'a> CommanderCache<'a> {
    pub fn new(commander: &'a Commander) -> CommanderCache {
        Self { commander }
    }
}

impl FileController for CommanderCache<'_> {
    fn key(&self, key: &str) -> String {
        format!(".cache/{}/{}.json", Self::file_safe_string(&self.commander.to_string()), key)
    }
}

pub trait Cacheable<'a, T: for<'b> Deserialize<'b> + Serialize>: Sized {
    type C: FileController;
    async fn compute(&self) -> anyhow::Result<T>;
    fn cache_file_path(&self) -> String;

    async fn load(&self, cache: &impl FileController) -> anyhow::Result<T> {
        cache.read::<T>(&self.cache_file_path()).await
    }

    async fn load_or_compute(&self, handler: &impl FileController) -> anyhow::Result<T> {
        if let Ok(data) = self.load(handler).await {
            return Ok(data);
        }

        let value = self.compute().await?;
        handler.write(&self.cache_file_path(), &value).await?;

        Ok(value)
    }
}