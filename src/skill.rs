use arcdps_parse::{BuffFormula, BuffInfo, Log, SkillInfo, SkillTiming};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillOrBuff {
    Skill(Skill),
    Buff(Buff),
}

impl From<Skill> for SkillOrBuff {
    fn from(skill: Skill) -> Self {
        Self::Skill(skill)
    }
}

impl From<Buff> for SkillOrBuff {
    fn from(buff: Buff) -> Self {
        Self::Buff(buff)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: Option<String>,
    pub info: SkillInfo,
    pub timings: Vec<SkillTiming>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buff {
    pub id: u32,
    pub name: Option<String>,
    pub info: BuffInfo,
    pub formulas: Vec<BuffFormula>,
}

pub fn extract_skill(log: &Log, id: u32) -> Option<SkillOrBuff> {
    let name = log.skill_name(id).map(Into::into);
    let iter = log.events.iter().filter(|event| event.skill_id == id);

    if let Some(info) = iter.clone().find_map(|event| event.skill_info()) {
        let timings = iter
            .clone()
            .filter_map(|event| event.skill_timing())
            .collect();

        Some(
            Skill {
                id,
                name,
                info,
                timings,
            }
            .into(),
        )
    } else if let Some(info) = iter.clone().find_map(|event| event.buff_info()) {
        let formulas = iter
            .clone()
            .filter_map(|event| event.buff_formula())
            .collect();

        Some(
            Buff {
                id,
                name,
                info,
                formulas,
            }
            .into(),
        )
    } else {
        None
    }
}
