use crate::command::Executor;
use dialoguer::FuzzySelect;
use store::Store;
use types::card::Card;
use types::deck_data_list::{DeckDataList, TopDeckMethod};

pub struct SearchDecksForCard {
    card: Option<Card>,
    method: Vec<TopDeckMethod>,
}

impl SearchDecksForCard {
    pub fn new(card: Option<Card>, method: Option<Vec<TopDeckMethod>>) -> Self {
        Self { card, method: method.unwrap_or(vec![TopDeckMethod::default()]) }
    }
}

impl Executor for SearchDecksForCard {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        let card = {
            if let Some(card) = self.card {
                card
            } else {
                let mut all_cards = Vec::from_iter(store.all_cards().await?.into_iter());
                all_cards.sort();
                let selection = FuzzySelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Which Card are you looking for?")
                    .items(&all_cards)
                    .interact()?;

                all_cards[selection].clone()
            }
        };

        let all_decks = store.all_decks().await?.into_top_decks_with_methods(&self.method[..]);
        let decks_with_card = all_decks.into_iter().filter(|deck| deck.list().contains(&card)).collect::<DeckDataList>();
        let common_cards_set = decks_with_card.common_cards();
        let mut common_cards = Vec::from_iter(common_cards_set.iter());
        common_cards.sort();

        println!("Decks with {card}: {}", decks_with_card.len());
        println!("Average of decks: {:.4}", decks_with_card.average());
        println!();

        println!("Decks have {} common cards", common_cards.len());
        println!();
        for card in common_cards.iter() {
            println!(" {card}");
        }
        println!();

        let all_cards_set = decks_with_card.all_cards();
        let mut all_cards = Vec::from_iter(all_cards_set.difference(&common_cards_set));
        all_cards.sort();

        println!("Decks have {} uncommon cards", all_cards.len());
        println!();
        for card in all_cards.iter() {
            println!(" {card}");
        }

        Ok(())
    }
}