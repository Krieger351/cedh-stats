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
    deck_data_list.get(get_selection(prompt, &deck_data_list, omit)).unwrap()
}

fn get_splits<'a>(one: &'a DeckData, two: &'a DeckData) -> (HashSet<&'a Card>, HashSet<&'a Card>, HashSet<&'a Card>, HashSet<&'a Card>, HashSet<&'a Card>) {
    let one_set = one.list().iter().collect::<HashSet<_>>();
    let two_set = two.list().iter().collect::<HashSet<_>>();

    let union = one_set.union(&two_set).map(|x| *x).collect::<HashSet<_>>();
    let intersection = one_set.intersection(&two_set).map(|x| *x).collect::<HashSet<_>>();
    let difference = one_set.symmetric_difference(&two_set).map(|x| *x).collect::<HashSet<_>>();
    let one_exclusive = difference.intersection(&one_set).map(|x| *x).collect::<HashSet<_>>();
    let two_exclusive = difference.intersection(&two_set).map(|x| *x).collect::<HashSet<_>>();

    (union, intersection, difference, one_exclusive, two_exclusive)
}

impl Executor for Compare {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        let all_decks = store.all_decks().await?;

        let one = get_id("First Id:", &all_decks, self.0, None);
        let two = get_id("Second Id:", &all_decks, self.1, Some(one.id()));

        let (union, intersection, difference, one_exclusive, two_exclusive) = get_splits(one, two);

        println!("Decks have {} total cards, {} intersecting cards, and {} different cards", style(union.len()).cyan(), style(intersection.len()).cyan(), style(difference.len()).cyan());
        println!("Ids      {:>30} {:>30}", one.id(), two.id());
        println!("Standing {:>30} {:>30}", one.standing(), two.standing());

        let style_common = Style::new().color256(33);
        let style_one = Style::new().color256(34);
        let style_two = Style::new().color256(124);

        println!("There are {} {} cards, deck {} has {} unique cards, deck {} has {} unique cards", intersection.len(), style_common.apply_to("common"), style_one.apply_to("one"), one_exclusive.len(), style_two.apply_to("two"), two_exclusive.len());

        let mut union_list = union.into_iter().collect::<Vec<_>>();
        union_list.sort();
        union_list.iter().for_each(|card| {
            let color = if one_exclusive.contains(card) {
                &style_one
            } else if two_exclusive.contains(card) {
                &style_two
            } else {
                &style_common
            };

            println!("{}", color.apply_to(card));
        });

        Ok(())
    }
}