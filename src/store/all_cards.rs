use crate::data_structures::DeckList;
use crate::store::Store;
use anyhow::Result;

impl Store {
    pub async fn all_cards(self: &Self) -> Result<DeckList> {
        if let Ok(data) = self.cache.read_commander("meta/all_cards").await {
            Ok(data)
        } else {
            let mut all_cards = DeckList::default();
            let ids = self.ids_with_deck_list().await?;

            for id in ids.iter() {
                let list = self.deck_list(id).await?.unwrap();

                all_cards.extend(list);
            }

            self.cache.write_commander("meta/all_cards", &all_cards).await?;
            Ok(all_cards)
        }
    }
}