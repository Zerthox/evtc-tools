use arcdps_log_tools::{extract_casts, extract_positions, extract_skill, SkillOrBuff};
use arcdps_parse::{Log, Skill};
use clap::{error::ErrorKind, CommandFactory, Parser};

mod args;

use self::args::*;

fn main() {
    let Cli { command, args } = Cli::parse();

    if args.input.is_empty() {
        Cli::command()
            .error(
                ErrorKind::MissingRequiredArgument,
                "input file was not provided",
            )
            .exit();
    }

    let log = args.open_log();
    let events = args.filter_log(&log);

    match command {
        Command::All => {
            let events: Vec<_> = events.map(|event| (event.kind(), event)).collect();
            println!("Found {} events", events.len());
            args.write_output(&events);
        }

        Command::Cast { skill } => {
            let skill = find_skill(&log, &skill);
            println!("Finding casts of skill \"{}\" ({})", skill.name, skill.id,);

            let data = extract_casts(&log, events, skill.id);
            println!(
                "Found {} casts and {} hits without cast",
                data.casts.len(),
                data.hits_without_cast.len()
            );

            args.write_output(&data);
        }

        Command::Skill { skill } => {
            let skill = find_skill(&log, &skill);
            println!("Finding data for skill \"{}\" ({})", skill.name, skill.id);

            match extract_skill(&log, skill.id) {
                Some(SkillOrBuff::Skill(skill)) => {
                    println!("Found skill with {} timings", skill.timings.len());
                    args.write_output(&skill);
                }
                Some(SkillOrBuff::Buff(buff)) => {
                    println!("Found buff with {} formulas", buff.formulas.len());
                    args.write_output(&buff);
                }
                None => println!("Found no information"),
            }
        }

        Command::Position => {
            println!("Finding positions");

            let positions = extract_positions(&log, events);
            println!("Found {} positions", positions.len());
            if let Some(pos) = positions.first() {
                println!("Initial position at {} {} {}", pos.x, pos.y, pos.z);
            }

            args.write_output(&positions);
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
