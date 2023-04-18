mod cast;
mod position;

pub use cast::*;
pub use position::*;

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
        Self::new(
            id,
            agent_name(&log.agents, id).and_then(|names| names.first()),
        )
    }
}

pub fn skill_name(skills: &[arcdps::Skill], id: u32) -> Option<&str> {
    skills.iter().find_map(|skill| {
        if skill.id == id {
            Some(skill.name.as_str())
        } else {
            None
        }
    })
}

pub fn agent_name(agents: &[arcdps::Agent], id: u64) -> Option<&[String]> {
    agents.iter().find_map(|agent| {
        if agent.address == id {
            Some(agent.name.as_slice())
        } else {
            None
        }
    })
}
