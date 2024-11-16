use crate::data_structures::Entry;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Entries(Vec<Entry>);

impl Entries {
    pub fn iter(&self) -> std::slice::Iter<'_, Entry> {
        self.0.iter()
    }
}

impl Extend<Entry> for Entries {
    fn extend<T: IntoIterator<Item=Entry>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}