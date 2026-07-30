use crate::{Agent, Time};
use evtc_parse::{Event, Log, position::PositionEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub time: i32,
    pub agent: Agent,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub fn extract_positions<'a>(
    log: &'a Log,
    events: impl Iterator<Item = &'a Event>,
) -> Vec<Position> {
    let start = Time::log_start(log);
    events
        .filter_map(|event| event.try_extract::<PositionEvent>())
        .map(|event| Position {
            time: start.relative(event.time),
            agent: Agent::from_agent(&event.agent, log),
            x: event.position.x,
            y: event.position.y,
            z: event.position.z,
        })
        .collect()
}
