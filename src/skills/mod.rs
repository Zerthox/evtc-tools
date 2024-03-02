use evtc_parse::{event as evtc, Log};
use serde::{Deserialize, Serialize};

mod buff;
mod skill;

pub use buff::*;
pub use skill::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillIdName {
    pub id: u32,
    pub name: Option<String>,
}

impl SkillIdName {
    pub fn from_log(log: &Log, id: u32) -> Self {
        Self {
            id,
            name: log.skill_name(id).map(Into::into),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillWithInfo {
    #[serde(flatten)]
    pub skill: SkillIdName,

    #[serde(flatten)]
    pub kind: Option<SkillKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SkillKind {
    Skill(SkillInfo),
    Buff(BuffInfo),
}

pub fn extract_skills(log: &Log, id: Option<u32>) -> Vec<SkillWithInfo> {
    log.skills
        .iter()
        .filter(|skill| match id {
            Some(id) => skill.id == id,
            None => true,
        })
        .map(|skill| {
            let id = skill.id;
            let name = skill.name.clone();
            let iter = log.events.iter().filter(|event| event.skill_id == skill.id);

            let kind = if let Some(info) = iter
                .clone()
                .find_map(|event| event.try_extract::<evtc::SkillInfo>())
            {
                let timings = iter
                    .clone()
                    .filter_map(|event| event.try_extract::<evtc::SkillTiming>());

                Some(SkillKind::Skill(SkillInfo::new(info, timings)))
            } else if let Some(info) = iter
                .clone()
                .find_map(|event| event.try_extract::<evtc::BuffInfo>())
            {
                let formulas = iter
                    .clone()
                    .filter_map(|event| event.try_extract::<evtc::BuffFormula>());

                Some(SkillKind::Buff(BuffInfo::new(info, formulas)))
            } else {
                None
            };

            SkillWithInfo {
                skill: SkillIdName {
                    id,
                    name: Some(name),
                },
                kind,
            }
        })
        .collect()
}
