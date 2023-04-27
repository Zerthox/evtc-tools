mod cast;
mod position;
mod skill;
mod util;

pub use self::cast::*;
pub use self::position::*;
pub use self::skill::*;

use arcdps_parse as arcdps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: u64,
    pub name: Option<String>,
}

impl Agent {
    pub fn new(id: u64, name: Option<impl Into<String>>) -> Self {
        Self {
            id,
            name: name.map(Into::into),
        }
    }

    pub fn from_log(id: u64, log: &arcdps::Log) -> Self {
        Self::new(id, log.agent_name(id).and_then(|names| names.first()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BreakbarState {
    Active = 0,
    Recover = 1,
    Immune = 2,
    None = 3,
}
