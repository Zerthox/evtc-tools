use clap::{error::ErrorKind, CommandFactory, Parser};
use evtc_parse::{EventKind, Log, Skill};
use evtc_tools::{
    extract_casts, extract_effects, extract_gear, extract_positions, extract_skills,
    hit_map::map_hits_to_set, missile::extract_missiles, Time,
};
use serde::{Deserialize, Serialize};

mod cli;

use self::cli::*;

fn main() {
    let args = Args::parse();

    if args.input.is_empty() {
        Args::command()
            .error(
                ErrorKind::MissingRequiredArgument,
                "input file was not provided",
            )
            .exit();
    }

    let log = args.open_log();
    let events = args.filter_log(&log);

    match &args.command {
        Command::All => {
            #[derive(Debug, Clone, Serialize, Deserialize)]
            struct Event {
                #[serde(skip_serializing_if = "Option::is_none")]
                time_since_start: Option<i32>,
                #[serde(flatten)]
                event: EventKind,
            }

            let start = Time::log_start(&log);
            let events: Vec<Event> = events
                .cloned()
                .map(|event| Event {
                    time_since_start: event.time().map(|time| start.relative(time)),
                    event: event.into_kind(),
                })
                .collect();
            println!("Found {} events", events.len());
            args.write_output(&events);
        }

        Command::Agents => {
            let agents = &log.agents;
            println!("Found {} agents", agents.len());
            args.write_output(agents);
        }

        Command::Skills { skill } => {
            let skill = skill.as_ref().map(|arg| find_skill(&log, arg));
            match skill {
                Some(skill) => println!("Finding skill data for \"{}\" ({})", skill.name, skill.id),
                None => println!("Finding all skill data"),
            }

            let skills = extract_skills(&log, skill.map(|skill| skill.id));
            println!("Found {} skills/buffs", skills.len());

            args.write_output(&skills);
        }

        Command::Casts { skill } => {
            let skill = skill.as_ref().map(|arg| find_skill(&log, arg));
            match skill {
                Some(skill) => println!("Finding casts of skill \"{}\" ({})", skill.name, skill.id),
                None => println!("Finding all skill casts"),
            }

            let data = extract_casts(&log, events, skill.map(|skill| skill.id));
            println!(
                "Found {} casts and {} hits without cast",
                data.casts.len(),
                data.hits_without_cast.len()
            );

            args.write_output(&data);
        }

        Command::Positions => {
            println!("Finding positions");

            let positions = extract_positions(&log, events);
            println!("Found {} positions", positions.len());
            if let Some(pos) = positions.first() {
                println!("Initial position at {} {} {}", pos.x, pos.y, pos.z);
            }

            args.write_output(&positions);
        }

        Command::Effects => {
            println!("Finding effects");

            let effects = extract_effects(&log, events);
            println!("Found {} effects", effects.len());

            args.write_output(&effects);
        }

        Command::Missiles => {
            println!("Finding missiles");

            let missiles = extract_missiles(&log, events);
            println!("Found {} missiles", missiles.len());

            args.write_output(&missiles);
        }

        Command::Hitmap => {
            let agent = args.agent_filter(&log).unwrap_or_else(|| {
                Args::command()
                    .error(
                        ErrorKind::MissingRequiredArgument,
                        "hit mapping requires agent",
                    )
                    .exit()
            });
            println!("Mapping direct damage hits to weapon sets");

            let hit_map = map_hits_to_set(&log, events, agent.id);
            println!("Found {} weapon sets", hit_map.len());

            args.write_output(&hit_map);
        }

        Command::Gear => {
            println!("Extracting POV gear");

            let gear = extract_gear(&log);
            println!(
                "Found {} runes, {} relics, {} sigils, {} unknown gear buffs",
                gear.runes.len(),
                gear.relics.len(),
                gear.sigils
                    .values()
                    .map(|sigils| sigils.len())
                    .sum::<usize>(),
                gear.other.len(),
            );

            args.write_output(&gear);
        }
    }
}

fn find_skill<'a>(log: &'a Log, id_or_name: &str) -> &'a Skill {
    log.skills
        .iter()
        .find(|entry| match id_or_name.parse::<u32>() {
            Ok(id) => entry.id == id,
            Err(_) => entry.name == id_or_name,
        })
        .unwrap_or_else(|| panic!("Skill \"{}\" not found", id_or_name))
}
