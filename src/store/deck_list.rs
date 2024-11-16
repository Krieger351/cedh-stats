use crate::data_structures::{DeckId, DeckList};
use crate::moxfield::Moxfield;
use crate::store::Store;
use anyhow::Result;

impl Store {
    pub async fn deck_list(self: &Self, id: &DeckId) -> Result<Option<DeckList>> {
        let key = &format!("deck_list/{}", id);
        if let Ok(data) = self.cache.read_commander(key).await {
            Ok(data)
        } else {
            let list = match Moxfield::get_list(id).await {
                Ok(response) => response,
                _ => None
            };

            self.cache.write_commander(key, &list).await?;
            Ok(list)
        }
    }
}