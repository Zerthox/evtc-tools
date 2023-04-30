use crate::{log_start, Agent};
use arcdps_parse::{CombatEvent, Log, StateChange};
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
    let start = log_start(log);
    events
        .filter(|event| event.is_statechange == StateChange::Position)
        .map(|event| {
            let pos = event.position().unwrap();
            Position {
                time: event.time - start,
                agent: Agent::from_log(event.src_agent, log),
                x: pos.x,
                y: pos.y,
                z: pos.z,
            }
        })
        .collect()
}
