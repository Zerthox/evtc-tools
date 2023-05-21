mod rune;
mod sigil;

pub use self::rune::*;
pub use self::sigil::*;

use crate::log_start;
use arcdps_parse::{CombatEvent, EventKind, Log, Profession, Specialization, StateChange};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearInfo {
    pub id: u64,
    pub instance_id: u16,
    pub character: String,
    pub account: String,
    pub profession: Profession,
    pub elite: Option<Specialization>,
    pub runes: HashSet<GearItem<Rune>>,
    pub sigils: HashSet<GearItem<Sigil>>,
    pub buffs: Vec<GearBuff>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearBuff {
    pub time: u64,
    pub id: u32,
    pub log_name: String,
}

impl PartialEq for GearBuff {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GearBuff {}

impl Hash for GearBuff {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearItem<T> {
    #[serde(flatten)]
    pub buff: GearBuff,
    pub item: T,
}

impl<T> PartialEq for GearItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.buff.eq(&other.buff)
    }
}

impl<T> Eq for GearItem<T> {}

impl<T> Hash for GearItem<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buff.hash(state)
    }
}

const BUFF_CATEGORY_CHANGE: u64 = 138680;

pub fn extract_gear<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a CombatEvent> + Clone,
) -> GearInfo {
    let start = log_start(log);

    let build = events
        .clone()
        .find(|event| event.is_statechange == StateChange::GWBuild)
        .map(|event| event.src_agent)
        .unwrap_or_default();

    let pov = events
        .clone()
        .find(|event| event.is_statechange == StateChange::PointOfView)
        .expect("no pov");
    let agent = log.agent(pov.src_agent).expect("no pov agent");

    let gear_infos: HashMap<_, _> = events
        .clone()
        .filter_map(|event| event.buff_info().map(|info| (event.skill_id, info)))
        .filter(|(_, info)| info.category == if build >= BUFF_CATEGORY_CHANGE { 7 } else { 6 })
        .collect();

    let applies: Vec<_> = events
        .filter(|event| {
            event.src_agent == pov.src_agent
                && (event.is_statechange == StateChange::BuffInitial
                    || event.kind() == EventKind::BuffApply)
        })
        .map(|event| {
            let id = event.skill_id;
            GearBuff {
                id,
                time: event.time - start,
                log_name: log.skill_name(id).unwrap_or_default().into(),
            }
        })
        .collect();

    let runes = applies
        .iter()
        .cloned()
        .filter_map(|buff| buff.id.try_into().ok().map(|item| GearItem { buff, item }))
        .collect();

    let sigils = applies
        .iter()
        .cloned()
        .filter_map(|buff| buff.id.try_into().ok().map(|item| GearItem { buff, item }))
        .collect();

    let buffs = applies
        .iter()
        .filter(|buff| gear_infos.contains_key(&buff.id))
        .cloned()
        .collect();

    GearInfo {
        id: pov.src_agent,
        instance_id: pov.src_instance_id,
        character: agent.name[0].clone(),
        account: agent.name[1].clone(),
        profession: agent.profession.into(),
        elite: match agent.is_elite.into() {
            Specialization::Unknown => None,
            elite => Some(elite),
        },
        runes,
        sigils,
        buffs,
    }
}
