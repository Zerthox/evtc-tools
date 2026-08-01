use crate::{Agent, util::to_tick_rounded};
use evtc_parse::{Log, event::MissileLaunch};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastMissile {
    pub tick: i32,
    pub time: i32,
    pub motion: u8,
    pub speed: i16,
    pub target: Option<Agent>,
}

impl CastMissile {
    pub fn new(time: i32, event: &MissileLaunch, log: &Log) -> Self {
        let target = Agent::from_agent(&event.target, log).non_empty();
        Self {
            tick: to_tick_rounded(time),
            time,
            motion: event.motion,
            speed: event.speed,
            target,
        }
    }
}
