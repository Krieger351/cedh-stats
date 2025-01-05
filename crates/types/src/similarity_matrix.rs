use crate::card::Card;
use crate::similarity_score::SimilarityScore;
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::{
    fmt::{Debug, Display, Formatter},
    ops::{Index, IndexMut},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityMatrix(Array2<SimilarityScore>);

impl Display for SimilarityMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for col in self.0.rows() {
            for score in col {
                write!(f, "{score:<4.4} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SimilarityMatrix {
    pub fn new(len: usize) -> SimilarityMatrix {
        SimilarityMatrix(Array2::<SimilarityScore>::zeros((len, len)))
    }

    pub fn max(&self) -> &SimilarityScore {
        let mut max = &SimilarityScore::ZERO;

        for score in self.0.iter() {
            if score != &SimilarityScore::SELF && score > max {
                max = score;
            }
        }

        max
    }

    pub fn compute_similarity_matrix(sets: &[HashSet<Card>]) -> SimilarityMatrix {
        let n = sets.len();
        let mut matrix = SimilarityMatrix::new(n);
        for i in 0..n {
            for j in 0..=i {
                if i == j {
                    matrix[[i, j]] = SimilarityScore::SELF;
                } else {
                    let similarity = SimilarityScore::jaccard_similarity(&sets[i], &sets[j]);
                    matrix[[i, j]] = similarity.clone();
                    matrix[[j, i]] = similarity;
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