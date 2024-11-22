use crate::command::Executor;
use crate::data_types::deck_id_set::DeckIdSet;
use crate::store::Store;

pub struct Work {}

impl Executor for Work {
    async fn exec(&self, store: &Store<'_>) -> anyhow::Result<()> {
        let full_deck_id_deck_list_map = store.full_deck_id_deck_list_map().await?;
        let valid_keys = full_deck_id_deck_list_map.keys().cloned().collect::<DeckIdSet>();
        println!("{}", valid_keys.iter().len());
        // let
        // let full_id_win_rate_map = store.deck_id_win_rate_map().await?;
        // let top_ids = full_id_win_rate_map.into_top_decks_by_quartile();
        // let top_ids = top_ids.keys().collect::<Vec<_>>(); //.keys().take(10).collect::<Vec<_>>().iter();
        // let mut deck_lists = Vec::new();
        // for &id in &top_ids {
        //     if let Some(deck_list) = store.deck_list(id).await? {
        //         deck_lists.push(deck_list)
        //     }
        // }
        // let start = SystemTime::now();
        // let matrix = SimilarityMatrix::compute_similarity_matrix(&deck_lists);
        // let end = SystemTime::now();
        //
        // println!("{}", matrix);
        // println!("{:?}", end.duration_since(start));
        // println!("{:?}", matrix.len());
        // println!("{:#?}", DeckListClusters::generate_clusters(&top_ids[..deck_lists.len()], &matrix, &SimilarityScore::from_f64(0.5)));

        Ok(())
    }
}