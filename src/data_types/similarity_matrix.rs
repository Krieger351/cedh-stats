use crate::data_types::deck_list::DeckList;
use crate::data_types::similarity_score::SimilarityScore;
use std::fmt::{Display, Formatter};
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct SimilarityMatrix(Vec<Vec<SimilarityScore>>);

impl Display for SimilarityMatrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|c| c.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" ")).collect::<Vec<_>>().join("\n"))
    }
}

impl SimilarityMatrix {
    pub fn new(len: usize) -> SimilarityMatrix {
        SimilarityMatrix(vec![vec![SimilarityScore::ZERO; len]; len])
    }

    pub fn compute_similarity_matrix(lists: &[DeckList]) -> SimilarityMatrix {
        let n = lists.len();
        let mut matrix = SimilarityMatrix::new(n);
        for i in 0..n {
            for j in 0..=i {
                if i == j {
                    matrix[i][j] = SimilarityScore::SELF;
                } else {
                    let similarity = SimilarityScore::jaccard_similarity(&lists[i], &lists[j]);
                    matrix[i][j] = similarity.clone();
                    matrix[j][i] = similarity;
                }
            }
        }
        matrix
    }
}

impl Index<usize> for SimilarityMatrix {
    type Output = Vec<SimilarityScore>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for SimilarityMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}