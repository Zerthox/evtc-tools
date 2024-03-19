mod infusion;
mod item;
mod relic;
mod rune;
mod sigil;
mod trinket;

pub use self::{infusion::*, item::*, relic::*, rune::*, sigil::*, trinket::*};

use crate::{Time, WeaponMap, WeaponSet};
use evtc_parse::buff::BuffInitialEvent;
use evtc_parse::event::BuffApplyEvent;
use evtc_parse::{
    buff::{BuffCategory, BuffCategoryOld, BuffInfo},
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
    pub infusions: Vec<GearItem<Infusion>>,
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

    let buff_initials = log
        .events
        .iter()
        .filter_map(|event| event.try_extract::<BuffInitialEvent>())
        .filter(|event| event.dst.id == agent.id)
        .map(|event| GearBuff {
            id: event.skill_id,
            time: start.relative(event.time),
            log_name: log.skill_name(event.skill_id).unwrap_or_default().into(),
        });

    let buff_applies = log
        .events
        .iter()
        .filter_map(|event| event.try_extract::<BuffApplyEvent>())
        .filter(|event| event.dst.id == agent.id)
        .map(|event| GearBuff {
            id: event.skill_id,
            time: start.relative(event.time),
            log_name: log.skill_name(event.skill_id).unwrap_or_default().into(),
        });

    let mut runes = HashSet::new();
    let mut relics = HashSet::new();
    let mut sigils = GearItemMap::new();
    let mut infusions = Vec::new();
    let mut other = Vec::new();
    for buff in buff_initials.chain(buff_applies) {
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
        } else if let Ok(infusion) = Infusion::try_from(id) {
            infusions.push(GearItem::new(buff, infusion));
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
        infusions,
        other,
    }
}
