mod infusion;
mod item;
mod relic;
mod rune;
mod sigil;
mod trinket;

pub use self::{infusion::*, item::*, relic::*, rune::*, sigil::*, trinket::*};

use crate::{Time, WeaponMap, WeaponSet};
use evtc_parse::{
    self as evtc,
    buff::{BuffApplyEvent, BuffCategory, BuffCategoryOld, BuffInfo, BuffInitialEvent},
    Log, Profession, Specialization, StateChange,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

const BUFF_CATEGORY_CHANGE: u64 = 138680;

pub type GearItemMap<T> = HashMap<WeaponSet, HashSet<GearItem<T>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearInfo {
    pub id: u64,
    pub instance_id: u16,
    pub character: String,
    pub account: String,
    pub profession: Profession,
    pub elite: Option<Specialization>,
    pub trinkets: Vec<GearItem<Trinket>>,
    pub runes: HashSet<GearItem<Rune>>,
    pub relics: HashSet<GearItem<Relic>>,
    pub sigils: GearItemMap<Sigil>,
    pub infusions: Vec<GearItem<Infusion>>,
    pub other: Vec<GearBuff>,
}

impl GearInfo {
    pub fn empty(agent: &evtc::Agent, pov: &evtc::Event) -> Self {
        debug_assert!(agent.id == pov.src_agent);
        Self {
            id: pov.src_agent,
            instance_id: pov.src_instance_id,
            character: agent.name[0].clone(),
            account: agent.name[1].clone(),
            profession: agent.profession.into(),
            elite: match agent.is_elite.into() {
                Specialization::Unknown => None,
                elite => Some(elite),
            },
            trinkets: Default::default(),
            runes: Default::default(),
            relics: Default::default(),
            sigils: Default::default(),
            infusions: Default::default(),
            other: Default::default(),
        }
    }

    fn gear_category(&self, log: &Log) -> u8 {
        let build = log
            .events
            .iter()
            .find(|event| event.get_statechange() == StateChange::GWBuild)
            .map(|event| event.src_agent)
            .unwrap_or_default();

        if build >= BUFF_CATEGORY_CHANGE {
            BuffCategory::Upgrade as u8
        } else {
            BuffCategoryOld::Upgrade as u8
        }
    }

    pub fn populate(&mut self, log: &Log) {
        let gear_category = self.gear_category(log);
        let start = Time::log_start(log);
        let weapon_map = WeaponMap::new(&log.events, self.id);
        let gear_infos: HashMap<_, _> = log
            .events
            .iter()
            .filter_map(|event| event.try_extract::<BuffInfo>())
            .filter(|info| info.category == gear_category)
            .map(|info| (info.skill_id, info))
            .collect();

        let buff_initials = log
            .events
            .iter()
            .filter_map(|event| event.try_extract::<BuffInitialEvent>())
            .filter(|event| event.dst.id == self.id)
            .map(|event| GearBuff {
                id: event.skill_id,
                time: start.relative(event.time),
                log_name: log.skill_name(event.skill_id).unwrap_or_default().into(),
            });
        let buff_applies = log
            .events
            .iter()
            .filter_map(|event| event.try_extract::<BuffApplyEvent>())
            .filter(|event| event.dst.id == self.id)
            .map(|event| GearBuff {
                id: event.skill_id,
                time: start.relative(event.time),
                log_name: log.skill_name(event.skill_id).unwrap_or_default().into(),
            });

        for buff in buff_initials.chain(buff_applies) {
            let id = buff.id;
            if let Ok(trinket) = Trinket::try_from(id) {
                self.trinkets.push(GearItem::new(buff, trinket));
            } else if let Ok(rune) = Rune::try_from(id) {
                self.runes.insert(GearItem::new(buff, rune));
            } else if let Ok(relic) = Relic::try_from(id) {
                self.relics.insert(GearItem::new(buff, relic));
            } else if let Ok(sigil) = Sigil::try_from(id) {
                let set = weapon_map
                    .set_at(start.absolute(buff.time))
                    .unwrap_or_default();
                self.sigils
                    .entry(set)
                    .or_default()
                    .insert(GearItem::new(buff, sigil));
            } else if let Ok(infusion) = Infusion::try_from(id) {
                self.infusions.push(GearItem::new(buff, infusion));
            } else if gear_infos.contains_key(&buff.id) {
                self.other.push(buff);
            }
        }
    }
}

pub fn extract_gear(log: &Log) -> GearInfo {
    let pov = log
        .events
        .iter()
        .find(|event| event.get_statechange() == StateChange::PointOfView)
        .expect("no pov");
    let agent = log.agent(pov.src_agent).expect("no pov agent");

    let mut info = GearInfo::empty(agent, pov);
    info.populate(log);
    info
}
