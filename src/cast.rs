use arcdps_parse::Strike;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cast {
    pub time: u64,
    pub id: u32,
    pub name: Option<String>,
    pub agent: Agent,
    pub hits: Vec<Hit>,
}

impl Cast {
    pub fn new(id: u32, name: Option<impl Into<String>>, agent: Agent, time: u64) -> Self {
        Self {
            id,
            name: name.map(Into::into),
            agent,
            time,
            hits: Vec::new(),
        }
    }
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hit {
    pub kind: Strike,
    pub damage: i32,
    pub time: u64,
}

impl Hit {
    pub fn new(kind: Strike, damage: i32, time: u64) -> Self {
        Self { kind, time, damage }
    }
}
