use crate::command::Executor;
use dialoguer::console::Style;
use dialoguer::theme::ColorfulTheme;
use std::cmp::Reverse;
use std::collections::HashSet;
use store::Store;
use types::card::Card;
use types::card_set::CardSet;
use types::clusters::Clusters;
use types::deck_data_list::{DeckDataList, TopDeckMethod};
use types::deck_id::DeckId;
use types::similarity_matrix::SimilarityMatrix;
use types::similarity_score::SimilarityScore;

#[derive(Debug)]
pub struct FindClusters {
    method: Vec<TopDeckMethod>,
    card_to_look_for: Option<Card>,
    threshold: SimilarityScore,
}
impl FindClusters {
    pub fn new(method: Option<Vec<TopDeckMethod>>, card_to_look_for: Option<Card>, similarity_score: Option<SimilarityScore>) -> Self {
        Self { method: method.unwrap_or_default(), card_to_look_for, threshold: similarity_score.unwrap_or_default() }
    }
}

fn get_selection<'a>(clusters: &'a Clusters, deck_data_list: &DeckDataList) -> &'a DeckId {
    let mut items = vec![];
    for (id, cluster) in clusters.iter() {
        if cluster.len() > 1 {
            items.push((id, cluster.len(), deck_data_list.clone().pick_decks(cluster).average()))
        }
    }

    items.sort_by_key(|x| Reverse(x.2));

    let print_items = items.iter().map(|(id, len, wr)| format!("{id} {len}: {wr:.3}")).collect::<Vec<_>>();
    let selection = dialoguer::Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose one:")
        .items(&print_items)
        .default(0)
        .interact()
        .unwrap();

    items[selection].0
}

impl Executor for FindClusters {
    async fn exec(self, store: &Store<'_>) -> anyhow::Result<()> {
        let entries = store.all_decks().await?.into_top_decks_with_methods(&self.method[..]);
        let (ids, lists): (Vec<_>, Vec<_>) = entries.into_iter().map(|x| (x.id().clone(), x.list().clone())).unzip();

        let lists: Vec<HashSet<Card>> = lists.into_iter().map(|list| HashSet::from_iter(list.into_iter())).collect();

        println!("There are {} lists after filtering by {}", lists.len(), self.method.iter().map(|w| w.to_string()).collect::<Vec<_>>().join(", "));

        let matrix = SimilarityMatrix::compute_similarity_matrix(&lists);

        let max_similarity = matrix.max();

        println!("Max Similarity: {max_similarity}");

        let clusters = Clusters::generate_overlapping_clusters(&ids, &matrix, &self.threshold);

        let largest_cluster = {
            let mut largest = 0;
            clusters.iter().for_each(|(_, cluster)| if cluster.len() > largest {
                largest = cluster.len()
            });
            largest
        };

        println!("Largest Cluster: {largest_cluster}");

        if let Some(card) = &self.card_to_look_for {
            println!("Clusters containing: {card}");
            let all_decks = store.all_decks().await?;
            clusters.clone().into_iter().for_each(|(id, c)| {
                if all_decks.clone().pick_decks(&c).common_cards().contains(card) {
                    println!("{id}");
                }
            });
        }

        let selection = get_selection(&clusters, &store.all_decks().await?);
        let cluster = &clusters[selection];
        let cluster_decks = store.all_decks().await?.pick_decks(cluster);

        let win_rate_per_card = cluster_decks.win_rate_per_card();
        let all_cards = cluster_decks.all_cards();
        let common_cards = cluster_decks.common_cards();
        let uncommon_cards = all_cards.difference(&common_cards).cloned().collect::<CardSet>();
        let mut all_cards_vec = all_cards.iter().collect::<Vec<_>>();
        all_cards_vec.sort();
        
        let style_common = Style::new().color256(33);
        let style_uncommon = Style::new().color256(34);

        println!("Cluster Info");
        println!("average win rate: {:.4}", cluster_decks.average());
        println!("average standing: {:.4}", cluster_decks.average_standing());
        println!("There are {:.4} {} cards and {:.4} {} cards", common_cards.len(), style_common.apply_to("common"), uncommon_cards.len(), style_uncommon.apply_to("uncommon"));
        println!();

        for card in all_cards_vec {
            let win_rate = win_rate_per_card.get(card).unwrap();
            let color = if common_cards.contains(card) {
                &style_common
            } else if uncommon_cards.contains(card) {
                &style_uncommon
            } else {
                unreachable!();
            };

            println!("{win_rate:.4}: {}", color.apply_to(card));
        }
        Ok(())
    }
}