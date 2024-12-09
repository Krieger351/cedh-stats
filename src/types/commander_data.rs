use crate::types::commander::Commander;
use crate::types::deck_id::DeckId;
use crate::types::win_rate::WinRate;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommanderData {
    commander: Commander,
    win_rates: HashMap<DeckId, WinRate>,
    list_ids: HashSet<DeckId>,
}