mod cast;

pub use cast::{Agent, Cast, Hit};

use arcdps_parse::{self as arcdps, Activation, BuffRemove, CombatEvent, Log, Skill, StateChange};
use std::collections::HashMap;

pub fn extract_casts(log: &Log, skill_filter: u32, agent_filter: Option<u64>) -> Vec<Cast> {
    let mut casts = HashMap::<_, Vec<_>>::new();

    for event in &log.events {
        let id = event.skill_id;
        if id == skill_filter && Some(event.src_agent as u64) == agent_filter {
            match event {
                CombatEvent {
                    is_statechange: StateChange::None,
                    is_activation: Activation::Start,
                    ..
                } => {
                    // activation start
                    let agent_id = event.src_agent as u64;
                    let agent = Agent::new(agent_id, agent_name(&log.agents, agent_id));
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
                    let agent_id = event.src_agent as u64;
                    match casts.get_mut(&agent_id).and_then(|casts| casts.last_mut()) {
                        Some(cast) => {
                            cast.hits
                                .push(Hit::new(event.result.into(), event.value, event.time))
                        }
                        None => {
                            // TODO: logging
                            eprintln!(
                                "hit of skill {} from agent {} at time {} without prior cast",
                                id, agent_id, event.time
                            );
                        }
                    }
                }

                _ => {}
            }
        }
    }

    let mut result: Vec<_> = casts.into_iter().flat_map(|(_, cast)| cast).collect();
    result.sort_by_key(|cast| cast.time);
    result
}

pub fn skill_name(skills: &[Skill], id: u32) -> Option<&str> {
    skills.iter().find_map(|skill| {
        if skill.id == id {
            Some(until_null(&skill.name))
        } else {
            None
        }
    })
}

pub fn agent_name(agents: &[arcdps::Agent], id: u64) -> Option<&str> {
    agents.iter().find_map(|agent| {
        if agent.address == id {
            Some(until_null(&agent.name))
        } else {
            None
        }
    })
}

pub fn until_null(string: &str) -> &str {
    let end = string.find('\0').unwrap_or(string.len());
    &string[..end]
}
