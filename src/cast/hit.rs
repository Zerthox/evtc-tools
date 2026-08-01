use crate::{Agent, Hit, SkillIdName, util::to_tick_rounded};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastHit {
    pub tick: i32,

    #[serde(flatten)]
    pub hit: Hit,
}

impl CastHit {
    pub fn new(hit: Hit) -> Self {
        Self {
            tick: to_tick_rounded(hit.time),
            hit,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandaloneHit {
    pub skill: SkillIdName,
    pub agent: Agent,

    #[serde(flatten)]
    pub hit: Hit,
}

impl StandaloneHit {
    pub fn new(skill: SkillIdName, agent: Agent, hit: Hit) -> Self {
        StandaloneHit { skill, agent, hit }
    }
}
