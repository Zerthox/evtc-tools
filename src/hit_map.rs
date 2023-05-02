use crate::{log_start, Hit, HitWithSkill, Skill};
use arcdps_parse::{EventKind, Log, StateChange, WeaponSet};
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

pub fn map_hits_to_set(log: &Log, agent: u64) -> impl Iterator<Item = WeaponSetHits> {
    let start = log_start(log);
    let mut sets: HashMap<WeaponSet, Vec<HitWithSkill>> = HashMap::new();
    let mut unknown = Vec::new();

    let swaps: Vec<_> = log
        .events
        .iter()
        .filter(|event| event.is_statechange == StateChange::WeaponSwap)
        .map(|event| {
            (
                event.time,
                WeaponSet::try_from(event.dst_agent).expect("unknown weapon set"),
            )
        })
        .filter(|(_, set)| is_real(*set))
        .collect();

    for event in &log.events {
        if event.kind() == EventKind::DirectDamage && event.src_agent == agent {
            let mut new = HitWithSkill {
                skill: Skill::from_log(log, event.skill_id),
                hit: Hit::try_from_event(log, event).unwrap(),
            };
            new.hit.time -= start;

            match swaps.binary_search_by_key(&event.time, |(time, _)| *time) {
                Err(0) => unknown.push(new),
                Ok(index) => {
                    let (_, set) = swaps[index];
                    sets.entry(set).or_default().push(new);
                }
                Err(index) => {
                    let (_, set) = swaps[index - 1];
                    sets.entry(set).or_default().push(new);
                }
            }
        }
    }

    iter::once(WeaponSetHits::new(None, unknown)).chain(
        sets.into_iter()
            .map(|(set, hits)| WeaponSetHits::new(Some(set), hits)),
    )
}

fn is_real(set: WeaponSet) -> bool {
    matches!(
        set,
        WeaponSet::Land1 | WeaponSet::Land2 | WeaponSet::Water1 | WeaponSet::Water2
    )
}
