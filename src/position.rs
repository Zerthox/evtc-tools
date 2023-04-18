use crate::Agent;
use arcdps_parse::{Log, StateChange};
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

pub fn extract_positions(log: &Log, agent_filter: Option<u64>) -> Vec<Position> {
    log.events
        .iter()
        .filter(|event| {
            agent_filter.map(|id| event.src_agent == id).unwrap_or(true)
                && event.is_statechange == StateChange::Position
        })
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
