use crate::data_types::deck_id::DeckId;
use crate::data_types::deck_id_set::DeckIdSet;
use crate::data_types::similarity_matrix::SimilarityMatrix;
use crate::data_types::similarity_score::SimilarityScore;
use std::collections::hash_map::{Entry, Iter};
use std::collections::HashMap;

#[derive(Debug)]
pub struct DeckListClusters(HashMap<DeckId, DeckIdSet>);

impl DeckListClusters {
    pub fn new<'a>() -> DeckListClusters {
        DeckListClusters(HashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub(crate) fn iter(&self) -> Iter<'_, DeckId, DeckIdSet> {
        self.0.iter()
    }

    fn entry<'a>(&mut self, key: DeckId) -> Entry<'_, DeckId, DeckIdSet> {
        self.0.entry(key)
    }

    fn insert(&mut self, id: DeckId, set: DeckIdSet) -> Option<DeckIdSet> {
        self.0.insert(id, set)
    }

    pub(crate) fn generate_overlapping_clusters<'a>(deck_id_list: &[&DeckId], similarity_matrix: &'a SimilarityMatrix, threshold: &'a SimilarityScore) -> DeckListClusters {
        let mut clusters = DeckListClusters::new();

        let n = deck_id_list.len();

        // Iterate over the similarity matrix
        for i in 0..n {
            for j in 0..n {
                if i != j && &similarity_matrix[i][j] >= threshold {
                    // Add i and j to the same cluster
                    clusters.entry(deck_id_list[i].clone()).or_insert_with(DeckIdSet::new).insert(deck_id_list[j].clone());
                    clusters.entry(deck_id_list[j].clone()).or_insert_with(DeckIdSet::new).insert(deck_id_list[i].clone());
                }
            }
        }

        clusters
    }

    pub(crate) fn generate_clusters<'a>(deck_id_list: &[&DeckId], similarity_matrix: &'a SimilarityMatrix, threshold: &'a SimilarityScore) -> DeckListClusters {
        let n = deck_id_list.len();
        let mut visited = vec![false; n];
        let mut clusters = DeckListClusters::new();

        for i in 0..n {
            if !visited[i] {
                let mut cluster = DeckIdSet::new();
                for j in 0..n {
                    if &similarity_matrix[i][j] >= threshold && !visited[j] {
                        cluster.insert(deck_id_list[j].clone());
                        visited[j] = true;
                    }
                }
                clusters.insert(deck_id_list[i].clone(), cluster);
            }
        }
        clusters
    }
}