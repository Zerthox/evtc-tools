use evtc_parse::{AgentId, Log};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: u64,
    pub inst_id: Option<u16>,
    pub name: Option<String>,
}

impl Agent {
    pub fn new(id: u64, inst_id: Option<u16>, name: Option<impl Into<String>>) -> Self {
        Self {
            id,
            inst_id,
            name: name.map(Into::into),
        }
    }

    pub fn from_agent(agent: &AgentId, log: &Log) -> Self {
        Self::new(
            agent.id,
            Some(agent.instance_id),
            log.agent_name(agent.id).and_then(|names| names.first()),
        )
    }

    pub fn from_id(id: u64, log: &Log) -> Self {
        Self::new(id, None, log.agent_name(id).and_then(|names| names.first()))
    }
}
