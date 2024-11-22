use crate::data_types::deck_list::DeckList;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

// pub const ZERO_SIMILARITY: SimilarityScore = SimilarityScore(0.0);
// pub const SELF_SIMILARITY: SimilarityScore = SimilarityScore(0.0);

#[derive(Debug, Clone, Serialize, Deserialize)]
#[repr(transparent)]
pub struct SimilarityScore(f64);
impl SimilarityScore {
    pub(crate) const ZERO: SimilarityScore = SimilarityScore(0.0);
    pub(crate) const SELF: SimilarityScore = SimilarityScore(1.0);
    pub fn from_f64(f: f64) -> SimilarityScore {
        SimilarityScore(f)
    }
    pub fn jaccard_similarity(set_a: &DeckList, set_b: &DeckList) -> SimilarityScore {
        let intersection: f64 = set_a.intersection(set_b).len() as f64;
        let union: f64 = set_a.union(set_b).len() as f64;
        if union == 0.0 {
            SimilarityScore(0.0)
        } else {
            SimilarityScore(intersection / union)
        }
    }
}

impl Display for SimilarityScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl PartialEq<Self> for SimilarityScore {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for SimilarityScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

