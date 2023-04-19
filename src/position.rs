use crate::Agent;
use arcdps_parse::{CombatEvent, Log, StateChange};
use serde::{Deserialize, Serialize};
use std::mem::transmute;

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
        .filter(|event| event.is_statechange == StateChange::Position)
        .map(|event| {
            let agent = Agent::from_log(event.src_agent, log);
            let [x, y] = unsafe { transmute::<_, [f32; 2]>(event.dst_agent) };

            #[allow(clippy::transmute_int_to_float)]
            let z = unsafe { transmute::<_, f32>(event.value) };

            Position {
                time: event.time,
                agent,
                x,
                y,
                z,
            }
        })
        .collect()
}
