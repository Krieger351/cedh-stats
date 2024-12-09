use crate::data::edh_top_sixteen::EdhTopSixteen;
use crate::store::Store;
use anyhow::Result;

impl Store<'_> {
    pub async fn fetch_commander_data(&self) -> Result<()> {
        EdhTopSixteen::get_entries(self.commander()).await?;
        Ok(())
    }
}