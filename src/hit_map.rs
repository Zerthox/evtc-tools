use crate::{Hit, HitWithSkill, Skill, Time, WeaponMap, WeaponSet};
use evtc_parse::{Event, EventCategory, Log};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponSetHits {
    pub set: WeaponSet,
    pub damage: i64,
    pub hit_count: usize,
    pub hits: Vec<HitWithSkill>,
}

impl WeaponSetHits {
    pub fn new(set: WeaponSet, hits: impl IntoIterator<Item = HitWithSkill>) -> Self {
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
    events: impl IntoIterator<Item = &'a Event>,
    agent: u64,
) -> Vec<WeaponSetHits> {
    let start = Time::log_start(log);
    let mut sets: HashMap<WeaponSet, Vec<HitWithSkill>> = HashMap::new();
    let weapons = WeaponMap::new(&log.events, agent);

    for event in events {
        if event.categorize() == EventCategory::Strike {
            let new = HitWithSkill {
                skill: Skill::from_log(log, event.skill_id),
                hit: Hit::try_from_event(log, event, start.relative(event.time)).unwrap(),
            };

            let set = weapons.set_at(event.time).unwrap_or(WeaponSet::Initial);
            sets.entry(set).or_default().push(new);
        }
    }

    let mut sets = sets
        .into_iter()
        .map(|(set, hits)| WeaponSetHits::new(set, hits))
        .collect::<Vec<_>>();
    sets.sort_by_key(|entry| {
        entry
            .hits
            .first()
            .map(|entry| entry.hit.time)
            .unwrap_or_default()
    });
    sets
}
