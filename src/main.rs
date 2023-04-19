use arcdps_log_tools::{extract_casts, extract_positions};
use arcdps_parse::{CombatEvent, Log, Parse};
use clap::{error::ErrorKind, CommandFactory, Parser};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};
use zip::ZipArchive;

/// CLI interface.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Command.
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub args: Args,
}

/// Arguments.
#[derive(Debug, Clone, clap::Args)]
struct Args {
    /// Path to input file.
    #[clap(global = true, default_value_t)]
    pub input: String,

    /// Path to output file, defaults to input filename.
    #[clap(global = true)]
    pub output: Option<PathBuf>,

    /// Id or name of agent to filter data for.
    #[clap(short, long, global = true)]
    pub agent: Option<String>,
}

#[derive(Debug, Clone, clap::Subcommand)]
enum Command {
    /// Extract cast & hit data.
    Cast {
        /// Id or name of skill to extract data for.
        #[clap(short, long)]
        skill: String,
    },

    /// Extract position data.
    Position,
}

fn main() {
    let cli = Cli::parse();

    if cli.args.input.is_empty() {
        Cli::command()
            .error(
                ErrorKind::MissingRequiredArgument,
                "input file was not provided",
            )
            .exit();
    }

    let log = open_log(&cli.args.input);
    let events = filter_log(&log, &cli.args);

    match cli.command {
        Command::Cast { skill: skill_arg } => {
            let skill = log
                .skills
                .iter()
                .find(|skill| match skill_arg.parse::<u32>() {
                    Ok(id) => skill.id == id,
                    Err(_) => skill.name == skill_arg,
                })
                .unwrap_or_else(|| panic!("Skill \"{}\" not found", skill_arg));

            println!("Finding casts of skill \"{}\" ({})", skill.name, skill.id,);

            let (casts, hits_without_cast) = extract_casts(&log, events, skill.id);

            for info in &hits_without_cast {
                eprintln!(
                    "Hit from \"{}\" ({}) at time {} without prior cast",
                    log.agent_name(info.agent)
                        .and_then(|names| names.first().map(|name| name.as_str()))
                        .unwrap_or_default(),
                    info.agent,
                    info.time
                );
            }
            println!(
                "Found {} casts and {} hits without cast",
                casts.len(),
                hits_without_cast.len()
            );

            write_output(&cli.args, &casts);
        }

        Command::Position => {
            println!("Finding positions");

            let positions = extract_positions(&log, events);
            println!("Found {} positions", positions.len());
            if let Some(pos) = positions.first() {
                println!("Initial position at {} {} {}", pos.x, pos.y, pos.z);
            }

            write_output(&cli.args, &positions);
        }
    }
}

fn open_log(input: impl AsRef<Path>) -> Log {
    let mut archive = ZipArchive::new(BufReader::new(
        File::open(input).expect("unable to open input file"),
    ))
    .expect("input log file not compressed");
    let mut file = archive.by_index(0).expect("input log file empty");

    let mut log = Log::parse(&mut file).expect("failed to parse EVTC log");
    log.events.sort_by_key(|event| event.time);
    log
}

fn filter_log<'a>(log: &'a Log, args: &Args) -> impl Iterator<Item = &'a CombatEvent> {
    let agent = args.agent.as_deref().map(|arg| {
        log.agents
            .iter()
            .find(|agent| match arg.parse::<u64>() {
                Ok(id) => agent.address == id,
                Err(_) => agent.name[0] == arg,
            })
            .unwrap_or_else(|| panic!("Agent \"{}\" not found", arg))
    });

    println!(
        "Agent filter: {}",
        agent
            .map(|agent| format!("\"{}\" ({})", agent.name[0], agent.address))
            .unwrap_or_else(|| "all agents".into())
    );

    let agent_filter = agent.map(|agent| agent.address);
    log.events
        .iter()
        .filter(move |event| agent_filter.map(|id| event.src_agent == id).unwrap_or(true))
}

fn write_output<T>(args: &Args, value: &T)
where
    T: ?Sized + serde::Serialize,
{
    let out = args
        .output
        .clone()
        .unwrap_or_else(|| {
            Path::new(&args.input)
                .file_name()
                .expect("input path is no file")
                .into()
        })
        .with_extension("json");

    let output = BufWriter::new(File::create(&out).expect("failed to create output file"));
    serde_json::to_writer_pretty(output, value).expect("failed to serialize data");
    println!("Saved data to \"{}\"", out.display());
}
