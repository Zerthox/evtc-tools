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

    pub fn from_log(id: u64, log: &evtc_parse::Log) -> Self {
        Self::new(id, log.agent_name(id).and_then(|names| names.first()))
    }
}
