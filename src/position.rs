use crate::Agent;
use arcdps_parse::{CombatEvent, Log};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub time: u64,
    pub agent: Agent,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn extract_positions<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a CombatEvent>,
) -> Vec<Position> {
    events
        .filter_map(|event| {
            event.position().map(|pos| Position {
                time: event.time,
                agent: Agent::from_log(event.src_agent, log),
                x: pos.x,
                y: pos.y,
                z: pos.z,
            })
        })
        .collect()
}
