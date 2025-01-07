use crate::command::Executor;
use dialoguer::console::{style, Style};
use dialoguer::theme::ColorfulTheme;
use std::collections::HashSet;
use store::Store;
use types::card::Card;
use types::deck_data::DeckData;
use types::deck_data_list::DeckDataList;
use types::deck_id::DeckId;

pub struct Compare(Option<DeckId>, Option<DeckId>);

impl Compare {
    pub fn new(one: Option<DeckId>, two: Option<DeckId>) -> Self {
        Self(one, two)
    }
}

fn get_selection<'deck_data>(prompt: &str, deck_data_list: &'deck_data DeckDataList, omit: Option<&DeckId>) -> &'deck_data DeckId {
    let mut items = vec![];
    for data in deck_data_list.iter() {
        if Some(data.id()) != omit {
            items.push(data.id())
        }
    }

    let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .items(&items)
        .default(0)
        .interact()
        .unwrap();

    items[selection]
}

fn get_id<'deck_data>(prompt: &str, deck_data_list: &'deck_data DeckDataList, id: Option<DeckId>, omit: Option<&DeckId>) -> &'deck_data DeckData {
    if let Some(id) = id {
        if let Some(data) = deck_data_list.get(&id) {
            return data;
        }
    }
    loop {
        match deck_data_list.get(get_selection(prompt, &deck_data_list, omit)) {
            None => {}
            Some(data) => break data
        }
    }
}

impl Executor for Compare {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        let all_decks = store.all_decks().await?;


        let one = get_id("First Id:", &all_decks, self.0, None);
        let two = get_id("Second Id:", &all_decks, self.1, Some(one.id()));

        let one_set = one.list().iter().collect::<HashSet<&Card>>();
        let two_set = two.list().iter().collect::<HashSet<&Card>>();

        let mut union = one_set.union(&two_set).map(|x| *x).collect::<HashSet<&Card>>().into_iter().collect::<Vec<_>>();
        union.sort();
        let intersection = one_set.intersection(&two_set).map(|x| *x).collect::<HashSet<&Card>>();
        let difference = one_set.difference(&two_set).map(|x| *x).collect::<HashSet<&Card>>();

        println!("Decks have {} total cards, {} intersecting cards, and {} different cards", style(union.len()).cyan(), style(intersection.len()).cyan(), style(difference.len()).cyan());
        println!("Ids      {:>30} {:>30}", one.id(), two.id());
        println!("Standing {:>30} {:>30}", one.standing(), two.standing());

        for card in union.iter() {
            let color = if one_set.get(card).is_some() && two_set.get(card).is_some() {
                Style::new().white()
            } else if one_set.get(card).is_some() {
                Style::new().blue()
            } else if two_set.get(card).is_some() {
                Style::new().red()
            } else {
                Style::new().black()
            };

            println!("{}", color.apply_to(card));
        }

        Ok(())
    }
}