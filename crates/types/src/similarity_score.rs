use crate::card::Card;
use anyhow::Context;
use num_traits::{Bounded, Zero};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::str::FromStr;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    ops::Add,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityScore(f64);

impl Default for SimilarityScore {
    fn default() -> Self {
        Self::DEFAULT.clone()
    }
}

impl SimilarityScore {
    pub const ZERO: SimilarityScore = SimilarityScore(0.0);
    pub const SELF: SimilarityScore = SimilarityScore(1.0);
    pub const DEFAULT: SimilarityScore = SimilarityScore(0.75);
    pub fn new(f: f64) -> Option<SimilarityScore> {
        if Self::min_value() <= f && Self::max_value() >= f {
            Some(Self(f))
        } else { None }
    }
    pub fn jaccard_similarity(set_a: &HashSet<Card>, set_b: &HashSet<Card>) -> SimilarityScore {
        let union: f64 = set_a.union(set_b).count() as f64;
        if union == 0.0 {
            SimilarityScore::ZERO
        } else {
            let intersection: f64 = set_a.intersection(set_b).count() as f64;
            SimilarityScore::new(intersection / union).unwrap_or(SimilarityScore::ZERO)
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
        self.eq(&other.0)
    }
}
impl PartialOrd for SimilarityScore {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl PartialEq<f64> for SimilarityScore {
    fn eq(&self, other: &f64) -> bool {
        &self.0 == other
    }
}

impl PartialOrd<f64> for SimilarityScore {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl FromStr for SimilarityScore {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.parse()?).context("out of bounds")
    }
}