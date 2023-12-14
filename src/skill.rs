use evtc_parse::{
    buff::{BuffFormula, BuffInfo},
    skill::{SkillInfo, SkillTiming},
    Log,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: u32,
    pub name: Option<String>,
}

impl Skill {
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
    pub skill: Skill,

    #[serde(flatten)]
    pub kind: Option<SkillKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SkillKind {
    Skill {
        #[serde(flatten)]
        info: SkillInfo,
        timings: Vec<SkillTiming>,
    },
    Buff {
        #[serde(flatten)]
        info: BuffInfo,
        formulas: Vec<BuffFormula>,
    },
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
                .find_map(|event| event.try_extract::<SkillInfo>())
            {
                let timings = iter
                    .clone()
                    .filter_map(|event| event.try_extract::<SkillTiming>())
                    .collect();

                Some(SkillKind::Skill { info, timings })
            } else if let Some(info) = iter
                .clone()
                .find_map(|event| event.try_extract::<BuffInfo>())
            {
                let formulas = iter
                    .clone()
                    .filter_map(|event| event.try_extract::<BuffFormula>())
                    .collect();

                Some(SkillKind::Buff { info, formulas })
            } else {
                None
            };

            SkillWithInfo {
                skill: Skill {
                    id,
                    name: Some(name),
                },
                kind,
            }
        })
        .collect()
}
