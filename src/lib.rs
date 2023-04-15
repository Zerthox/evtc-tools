mod cast;

pub use cast::{Agent, Cast, Hit};

use arcdps_parse::{self as arcdps, Activation, BuffRemove, CombatEvent, Log, Skill, StateChange};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct HitWithoutCast {
    pub id: u32,
    pub agent: u64,
    pub time: u64,
}

// TODO: include quickness gained/lost
#[allow(unused)]
const QUICKNESS: u32 = 1187;

pub fn extract_casts(
    log: &Log,
    skill_filter: u32,
    agent_filter: Option<u64>,
) -> (Vec<Cast>, Vec<HitWithoutCast>) {
    let mut casts = HashMap::<_, Vec<_>>::new();
    let mut hits_without_cast = Vec::new();

    for event in &log.events {
        let id = event.skill_id;
        if id == skill_filter && agent_filter.map(|id| event.src_agent == id).unwrap_or(true) {
            match event {
                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::Start,
                    ..
                } => {
                    // activation start
                    let agent_id = event.src_agent;
                    let agent = Agent::new(
                        agent_id,
                        agent_name(&log.agents, agent_id).and_then(|names| names.first()),
                    );
                    let cast = Cast::new(id, skill_name(&log.skills, id), agent, event.time);
                    casts.entry(agent_id).or_default().push(cast);
                }

                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::None,
                    is_buff_remove: BuffRemove::None,
                    buff: 0,
                    ..
                } => {
                    // direct damage
                    let agent_id = event.src_agent;
                    match casts.get_mut(&agent_id).and_then(|casts| casts.last_mut()) {
                        Some(cast) => cast.add_hit(event.result.into(), event.value, event.time),
                        None => hits_without_cast.push(HitWithoutCast {
                            id,
                            agent: agent_id,
                            time: event.time,
                        }),
                    }
                }

                _ => {}
            }
        }
    }

    let mut casts: Vec<_> = casts.into_iter().flat_map(|(_, cast)| cast).collect();
    casts.sort_by_key(|cast| cast.time);

    (casts, hits_without_cast)
}

pub fn skill_name(skills: &[Skill], id: u32) -> Option<&str> {
    skills.iter().find_map(|skill| {
        if skill.id == id {
            Some(skill.name.as_str())
        } else {
            None
        }
    })
}

pub fn agent_name(agents: &[arcdps::Agent], id: u64) -> Option<&[String]> {
    agents.iter().find_map(|agent| {
        if agent.address == id {
            Some(agent.name.as_slice())
        } else {
            None
        }
    })
}
