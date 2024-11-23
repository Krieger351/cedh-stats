use crate::command::Executor;
use crate::data_types::deck_id_deck_list_map::DeckIdDeckListMap;
use crate::data_types::deck_list_clusters::DeckListClusters;
use crate::data_types::similarity_matrix::SimilarityMatrix;
use crate::data_types::similarity_score::SimilarityScore;
use crate::store::Store;

pub struct Work {}

impl Executor for Work {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let full_deck_id_deck_list_map = store.full_deck_id_deck_list_map().await?;

        let full_id_win_rate_map = store.full_deck_id_win_rate_map().await?;

        let top_ids = full_id_win_rate_map.into_top_decks_by_percent();

        let deck_lists = full_deck_id_deck_list_map.into_iter().filter(|x| top_ids.get(&x.0).is_some()).collect::<DeckIdDeckListMap>();
        // println!("{:#?}", deck_lists);
        let top_ids = top_ids.keys().collect::<Vec<_>>(); //.keys().take(10).collect::<Vec<_>>().iter();
        // let mut deck_lists = Vec::new();
        // for &id in &top_ids {
        //     if let Some(deck_list) = store.deck_list(id).await? {
        //         deck_lists.push(deck_list)
        //     }
        // }
        // let start = SystemTime::now();
        let matrix = SimilarityMatrix::compute_similarity_matrix(&deck_lists.values().cloned().collect::<Vec<_>>()[..]);
        // let end = SystemTime::now();
        //
        // println!("{}", matrix);
        // println!("{:?}", end.duration_since(start));
        // println!("{:?}", matrix);
        println!("{:#?}", DeckListClusters::generate_clusters(&top_ids[..], &matrix, &SimilarityScore::from_f64(0.5)));

        Ok(())
    }
}