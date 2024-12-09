use crate::types::card_set::CardSet;
use num_traits::{Bounded, Zero};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    ops::Add,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore(f64);
impl SimilarityScore {
    pub const ZERO: SimilarityScore = SimilarityScore(0.0);
    pub const SELF: SimilarityScore = SimilarityScore(1.0);
    pub fn new(f: f64) -> Option<SimilarityScore> {
        if Self::min_value().0 < f && Self::max_value().0 > f {
            Some(Self(f))
        } else { None }
    }
    pub fn jaccard_similarity(set_a: &CardSet, set_b: &CardSet) -> SimilarityScore {
        let union: f64 = set_a.union(set_b).count() as f64;
        if union == 0.0 {
            SimilarityScore::ZERO
        } else {
            let intersection: f64 = set_a.intersection(set_b).count() as f64;
            SimilarityScore::new(intersection / union).unwrap()
        }
    }
}

impl Bounded for SimilarityScore {
    fn min_value() -> Self {
        Self::ZERO
    }

    fn max_value() -> Self {
        Self::SELF
    }
}

impl Add<Self> for SimilarityScore {
    type Output = SimilarityScore;

    fn add(self, rhs: Self) -> Self::Output {
        SimilarityScore(self.0 + rhs.0)
    }
}

impl Zero for SimilarityScore {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        self.0 == 0.0
    }
}

impl Display for SimilarityScore {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl PartialEq for SimilarityScore {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl PartialOrd for SimilarityScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

