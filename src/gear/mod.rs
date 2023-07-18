mod rune;
mod sigil;

pub use self::rune::*;
pub use self::sigil::*;

use crate::{Time, WeaponMap, WeaponSet};
use arcdps_parse::{EventKind, Log, Profession, Specialization, StateChange};
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
    pub sigils: GearItemMap<Sigil>,
    pub buffs: Vec<GearBuff>,
}

pub type GearItemMap<T> = HashMap<WeaponSet, HashSet<GearItem<T>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearBuff {
    pub time: i32,
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

impl<T> TryFrom<GearBuff> for GearItem<T>
where
    T: TryFrom<u32>,
{
    type Error = T::Error;

    fn try_from(buff: GearBuff) -> Result<Self, Self::Error> {
        buff.id.try_into().map(|item| Self { buff, item })
    }
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

pub fn extract_gear(log: &Log) -> GearInfo {
    let start = Time::log_start(log);

    let build = log
        .events
        .iter()
        .find(|event| event.is_statechange == StateChange::GWBuild)
        .map(|event| event.src_agent)
        .unwrap_or_default();

    let pov = log
        .events
        .iter()
        .find(|event| event.is_statechange == StateChange::PointOfView)
        .expect("no pov");
    let agent = log.agent(pov.src_agent).expect("no pov agent");

    let weapon_map = WeaponMap::new(&log.events);

    let gear_infos: HashMap<_, _> = log
        .events
        .iter()
        .filter_map(|event| event.buff_info().map(|info| (event.skill_id, info)))
        .filter(|(_, info)| info.category == if build >= BUFF_CATEGORY_CHANGE { 7 } else { 6 })
        .collect();

    let applies: Vec<_> = log
        .events
        .iter()
        .filter(|event| {
            event.src_agent == pov.src_agent
                && (event.is_statechange == StateChange::BuffInitial
                    || event.kind() == EventKind::BuffApply)
        })
        .map(|event| {
            let id = event.skill_id;
            GearBuff {
                id,
                time: start.relative(event.time),
                log_name: log.skill_name(id).unwrap_or_default().into(),
            }
        })
        .collect();

    let runes = applies
        .iter()
        .cloned()
        .filter_map(|buff| buff.try_into().ok())
        .collect();

    let mut sigils = GearItemMap::new();
    for buff in &applies {
        if let Ok(item) = GearItem::try_from(buff.clone()) {
            let set = weapon_map
                .set_at(start.absolute(item.buff.time))
                .unwrap_or_default();
            sigils.entry(set).or_default().insert(item);
        }
    }

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
