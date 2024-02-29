mod infusion;
mod item;
mod relic;
mod rune;
mod sigil;

#[allow(unused_imports)]
pub use self::infusion::*;
pub use self::item::*;
pub use self::relic::*;
pub use self::rune::*;
pub use self::sigil::*;

use crate::{Time, WeaponMap, WeaponSet};
use evtc_parse::event::BuffApplyEvent;
use evtc_parse::event::BuffInfo;
use evtc_parse::{
    buff::{BuffCategory, BuffCategoryOld},
    Log, Profession, Specialization, StateChange,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearInfo {
    pub id: u64,
    pub instance_id: u16,
    pub character: String,
    pub account: String,
    pub profession: Profession,
    pub elite: Option<Specialization>,
    pub runes: HashSet<GearItem<Rune>>,
    pub relics: HashSet<GearItem<Relic>>,
    pub sigils: GearItemMap<Sigil>,
    pub other: Vec<GearBuff>,
}

pub type GearItemMap<T> = HashMap<WeaponSet, HashSet<GearItem<T>>>;

const BUFF_CATEGORY_CHANGE: u64 = 138680;

pub fn extract_gear(log: &Log) -> GearInfo {
    let start = Time::log_start(log);

    let build = log
        .events
        .iter()
        .find(|event| event.get_statechange() == StateChange::GWBuild)
        .map(|event| event.src_agent)
        .unwrap_or_default();
    let gear_category = if build >= BUFF_CATEGORY_CHANGE {
        BuffCategory::Upgrade as u8
    } else {
        BuffCategoryOld::Upgrade as u8
    };

    let pov = log
        .events
        .iter()
        .find(|event| event.get_statechange() == StateChange::PointOfView)
        .expect("no pov");
    let agent = log.agent(pov.src_agent).expect("no pov agent");

    let weapon_map = WeaponMap::new(&log.events, agent.id);

    let gear_infos: HashMap<_, _> = log
        .events
        .iter()
        .filter_map(|event| event.try_extract::<BuffInfo>())
        .map(|info| (info.skill_id, info))
        .filter(|(_, info)| info.category == gear_category)
        .collect();

    let buff_applies: Vec<_> = log
        .events
        .iter()
        .filter_map(|event| event.try_extract::<BuffApplyEvent>())
        .map(|event| {
            let id = event.skill_id;
            GearBuff {
                id,
                time: start.relative(event.time),
                log_name: log.skill_name(id).unwrap_or_default().into(),
            }
        })
        .collect();

    let mut runes = HashSet::new();
    let mut relics = HashSet::new();
    let mut sigils = GearItemMap::new();
    let mut other = Vec::new();
    for buff in buff_applies.into_iter() {
        let id = buff.id;
        if let Ok(rune) = Rune::try_from(id) {
            runes.insert(GearItem::new(buff, rune));
        } else if let Ok(relic) = Relic::try_from(id) {
            relics.insert(GearItem::new(buff, relic));
        } else if let Ok(sigil) = Sigil::try_from(id) {
            let set = weapon_map
                .set_at(start.absolute(buff.time))
                .unwrap_or_default();
            sigils
                .entry(set)
                .or_default()
                .insert(GearItem::new(buff, sigil));
        } else if gear_infos.contains_key(&buff.id) {
            other.push(buff);
        }
    }

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
        relics,
        sigils,
        other,
    }
}
