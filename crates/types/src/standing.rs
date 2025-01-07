use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::ops::AddAssign;

#[derive(Serialize, Deserialize, Debug, Clone, PartialOrd, PartialEq)]
pub struct Standing(usize);

impl Display for Standing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Default for Standing {
    fn default() -> Self {
        Self(16)
    }
}

impl From<usize> for Standing {
    fn from(value: usize) -> Self {
        Self(value)
    }
}


impl AddAssign<&Standing> for Standing {
    fn add_assign(&mut self, rhs: &Standing) {
        *self = Self(self.0 + rhs.0);
    }
}