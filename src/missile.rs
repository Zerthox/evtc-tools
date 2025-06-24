use crate::Time;
use evtc_parse::{
    event::{MissileCreate, MissileLaunch, MissileRemove},
    Event, EventKind, Log,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: time as signed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Missile {
    pub create: Option<MissileCreate>,
    pub launches: Vec<MissileLaunch>,
    pub removes: Vec<MissileRemove>,
}

impl Missile {
    pub fn empty() -> Self {
        Self {
            create: None,
            launches: Vec::new(),
            removes: Vec::new(),
        }
    }

    pub fn create(mut create: MissileCreate, start: Time) -> Self {
        create.time = start.relative_saturate(create.time);
        Self {
            create: Some(create),
            ..Self::empty()
        }
    }

    pub fn add_launch(&mut self, mut launch: MissileLaunch, start: Time) {
        launch.time = start.relative_saturate(launch.time);
        self.launches.push(launch);
    }

    pub fn add_remove(&mut self, mut remove: MissileRemove, start: Time) {
        remove.time = start.relative_saturate(remove.time);
        self.removes.push(remove);
    }

    pub fn first_aware(&self) -> u64 {
        if let Some(create) = &self.create {
            create.time
        } else {
            self.launches
                .iter()
                .map(|launch| launch.time)
                .chain(self.removes.iter().map(|remove| remove.time))
                .min()
                .unwrap_or_default()
        }
    }
}

pub fn extract_missiles<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a Event> + Clone,
) -> Vec<Missile> {
    let start = Time::log_start(log);
    let mut missiles = HashMap::<u32, Vec<Missile>>::new();

    for event in events {
        match event.clone().into_kind() {
            EventKind::MissileCreate(create) => {
                missiles
                    .entry(create.tracking_id)
                    .or_default()
                    .push(Missile::create(create, start));
            }
            EventKind::MissileLaunch(launch) => {
                get_or_insert(&mut missiles, launch.tracking_id).add_launch(launch, start);
            }
            EventKind::MissileRemove(remove) => {
                get_or_insert(&mut missiles, remove.tracking_id).add_remove(remove, start);
            }
            _ => {}
        }
    }

    let mut vec: Vec<_> = missiles.into_values().flatten().collect();
    vec.sort_by_key(|missile| missile.first_aware());
    vec
}

fn get_or_insert(map: &mut HashMap<u32, Vec<Missile>>, id: u32) -> &mut Missile {
    let entry = map.entry(id).or_default();
    if entry.is_empty() {
        entry.push(Missile::empty());
    }
    entry.last_mut().unwrap()
}
