use crate::deck_id::DeckId;
use crate::deck_id_set::DeckIdSet;
use crate::similarity_matrix::SimilarityMatrix;
use crate::similarity_score::SimilarityScore;
use std::collections::hash_map::{Entry, IntoIter, Iter};
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Clusters(HashMap<DeckId, DeckIdSet>);

impl Clusters {
    pub fn new<'a>() -> Self {
        Self(HashMap::new())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> Iter<'_, DeckId, DeckIdSet> {
        self.0.iter()
    }

    fn entry<'a>(&mut self, key: DeckId) -> Entry<'_, DeckId, DeckIdSet> {
        self.0.entry(key)
    }

    fn insert(&mut self, id: DeckId, set: DeckIdSet) -> Option<DeckIdSet> {
        self.0.insert(id, set)
    }

    pub fn generate_overlapping_clusters(deck_id_list: &[DeckId], similarity_matrix: &SimilarityMatrix, threshold: &SimilarityScore) -> Self {
        let mut clusters = Self::new();

        let n = deck_id_list.len();

        // Iterate over the similarity matrix
        for i in 0..n {
            for j in 0..n {
                if &similarity_matrix[[i, j]] >= threshold {
                    // Add i and j to the same cluster
                    clusters.entry(deck_id_list[i].clone()).or_insert_with(DeckIdSet::new).insert(deck_id_list[j].clone());
                    clusters.entry(deck_id_list[j].clone()).or_insert_with(DeckIdSet::new).insert(deck_id_list[i].clone());
                }
            }
        }

        clusters
    }

    pub fn generate_clusters<'a>(deck_id_list: &[&DeckId], similarity_matrix: &'a SimilarityMatrix, threshold: &'a SimilarityScore) -> Self {
        let n = deck_id_list.len();
        let mut visited = vec![false; n];
        let mut clusters = Self::new();

        for i in 0..n {
            if !visited[i] {
                let mut cluster = DeckIdSet::new();
                for j in 0..n {
                    if &similarity_matrix[[i, j]] >= threshold && !visited[j] {
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

impl Index<&DeckId> for Clusters {
    type Output = DeckIdSet;

    fn index(&self, index: &DeckId) -> &Self::Output {
        &self.0[index]
    }
}

impl IntoIterator for Clusters {
    type Item = (DeckId, DeckIdSet);
    type IntoIter = IntoIter<DeckId, DeckIdSet>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}