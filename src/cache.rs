use crate::data_structures::Commander;
use anyhow::Result;
use serde::de;
use serde::ser;
use std::path::Path;

pub struct Cache {
    commander_name: Commander,
}

impl Cache {
    pub fn new(commander_name: &Commander) -> Cache {
        Cache {
            commander_name: commander_name.clone()
        }
    }

    fn full_key(self: &Self, key: &str) -> String {
        format!(".cache/{}.json", key)
    }
    fn full_commander_key(self: &Self, key: &str) -> String {
        self.full_key(&format!("{}/{}", self.commander_name, key.to_string()))
    }

    async fn internal_read<T: for<'a> de::Deserialize<'a>>(self: &Self, key: &str) -> Result<T> {
        let file_string = tokio::fs::read_to_string(key).await?.to_string();
        let data = serde_json::from_str::<T>(&file_string)?;
        Ok(data)
    }

    pub async fn read_commander<T: for<'a> de::Deserialize<'a>>(self: &Self, key: &str) -> Result<T> {
        self.internal_read::<T>(&self.full_commander_key(key)).await
    }

    pub async fn read<T: for<'a> de::Deserialize<'a>>(self: &Self, key: &str) -> Result<T> {
        self.internal_read::<T>(&self.full_key(key)).await
    }
 
    async fn internal_write<T: ser::Serialize>(self: &Self, key: &str, data: T) -> Result<()> {
        let data_string = serde_json::to_string_pretty::<T>(&data)?;
        tokio::fs::create_dir_all(Path::new(&key).parent().unwrap()).await?;
        tokio::fs::write(key, data_string).await?;
        Ok(())
    }

    pub async fn write_commander<T: ser::Serialize>(self: &Self, key: &str, data: T) -> Result<()> {
        self.internal_write::<T>(&self.full_commander_key(key), data).await
    }

    pub async fn write<T: ser::Serialize>(self: &Self, key: &str, data: T) -> Result<()> {
        self.internal_write::<T>(&self.full_key(key), data).await
    }
}