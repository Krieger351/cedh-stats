use crate::types::card_set::CardSet;
use crate::types::similarity_score::SimilarityScore;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityMatrix(Array2<SimilarityScore>);

impl Display for SimilarityMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl SimilarityMatrix {
    pub fn new(len: usize) -> SimilarityMatrix {
        SimilarityMatrix(Array2::<SimilarityScore>::zeros((len, len)))
    }

    pub fn compute_similarity_matrix(sets: &[CardSet]) -> SimilarityMatrix {
        let n = sets.len();
        let mut matrix = SimilarityMatrix::new(n);
        for i in 0..n {
            for j in 0..=i {
                if i == j {
                    matrix[[i, j]] = SimilarityScore::SELF;
                } else {
                    let similarity = SimilarityScore::jaccard_similarity(&sets[i], &sets[j]);
                    matrix[[i, j]] = similarity.clone();
                    matrix[[i, j]] = similarity;
                }
            }
        }
        matrix
    }
}

impl Index<[usize; 2]> for SimilarityMatrix {
    type Output = SimilarityScore;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<[usize; 2]> for SimilarityMatrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[index]
    }
}