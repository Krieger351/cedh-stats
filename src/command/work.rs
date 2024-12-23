use crate::command::Executor;
// use crate::data_types::deck_id_deck_list_map::DeckIdDeckListMap;
// use crate::data_types::deck_list_clusters::DeckListClusters;
use crate::store::Store;

pub struct Work {}

impl Executor for Work {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        // let mut full_deck_id_deck_list_map = store.full_deck_id_deck_list_map().await?;
        //
        // let full_id_win_rate_map = store.full_deck_id_win_rate_map().await?;
        //
        // let top_ids = full_id_win_rate_map.clone().into_top_decks_by_positive(); //.into_top_decks_by_quartile();
        //
        // println!("Win rate for top ids: {:.2}", top_ids.average_win_rate());
        //
        // let deck_lists = full_deck_id_deck_list_map.clone().into_iter().filter(|x| top_ids.get(&x.0).is_some()).collect::<DeckIdDeckListMap>();
        // let top_ids = &deck_lists.keys().collect::<Vec<_>>()[..];
        // let matrix_input = &deck_lists.values().cloned().map(|x1| x1.into_iter().collect::<CardSet>()).collect::<Vec<CardSet>>()[..];
        // assert_eq!(top_ids.len(), matrix_input.len());
        // let matrix = SimilarityMatrix::compute_similarity_matrix(matrix_input);
        //
        // let clusters = DeckListClusters::generate_overlapping_clusters(top_ids, &matrix, &SimilarityScore::new(0.7).unwrap());
        //
        // println!("There are {} clusters", clusters.len());
        // for (deck_id, deck_id_set) in clusters.iter() {
        //     if deck_id_set.len() > 20 {
        //         let mut some_ids = full_id_win_rate_map.clone();
        //         some_ids.retain_by_deck_id_set(deck_id_set);
        //         println!("{}({}): {:.2}", deck_id, some_ids.len(), some_ids.average_win_rate());
        //
        //         // let mut cluster = full_deck_id_deck_list_map.clone();
        //         // cluster.retain_by_key(deck_id_set);
        //         // cluster.into_all_cards().iter().for_each(|x1|
        //         //     println!("\t{}", x1)
        //         // );
        //
        //         println!()
        //     }
        // }

        Ok(())
    }
}