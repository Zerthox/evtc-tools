use crate::{Hit, HitWithSkill, Skill, Time, WeaponMap, WeaponSet};
use arcdps_parse::{CombatEvent, EventKind, Log};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponSetHits {
    pub set: Option<WeaponSet>,
    pub damage: i64,
    pub hit_count: usize,
    pub hits: Vec<HitWithSkill>,
}

impl WeaponSetHits {
    pub fn new(set: Option<WeaponSet>, hits: impl IntoIterator<Item = HitWithSkill>) -> Self {
        let hits: Vec<_> = hits.into_iter().collect();
        Self {
            set,
            damage: hits.iter().map(|hit| hit.hit.damage as i64).sum(),
            hit_count: hits.len(),
            hits,
        }
    }
}

pub fn map_hits_to_set<'a>(
    log: &'a Log,
    events: impl IntoIterator<Item = &'a CombatEvent>,
    agent: u64,
) -> impl Iterator<Item = WeaponSetHits> {
    let start = Time::log_start(log);
    let mut sets: HashMap<WeaponSet, Vec<HitWithSkill>> = HashMap::new();
    let mut unknown = Vec::new();

    let weapons = WeaponMap::new(&log.events, agent);

    for event in events {
        if event.kind() == EventKind::DirectDamage && event.src_agent == agent {
            let new = HitWithSkill {
                skill: Skill::from_log(log, event.skill_id),
                hit: Hit::try_from_event(log, event, start.relative(event.time)).unwrap(),
            };

            match weapons.set_at(event.time) {
                None => unknown.push(new),
                Some(set) => sets.entry(set).or_default().push(new),
            }
        }
    }

    iter::once(WeaponSetHits::new(None, unknown)).chain(
        sets.into_iter()
            .map(|(set, hits)| WeaponSetHits::new(Some(set), hits)),
    )
}
