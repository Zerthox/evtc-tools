use arcdps_hit_times::{agent_name, extract_casts, extract_positions};
use arcdps_parse::{Log, Parse};
use clap::Parser;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::PathBuf,
};
use zip::ZipArchive;

/// CLI arguments.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to input file.
    pub input: PathBuf,

    /// Id or name of agent to filter data for.
    #[clap(short, long)]
    pub agent: Option<String>,

    /// Path to output file, defaults to input filename.
    pub output: Option<PathBuf>,

    /// Id or name of skill to extract data for.
    #[clap(short, long)]
    skill: Option<String>,

    /// Extract position data.
    #[clap(short, long)]
    position: bool,
}

fn main() {
    let args = Args::parse();
    let out = args
        .output
        .unwrap_or_else(|| {
            args.input
                .file_name()
                .expect("input path is no file")
                .into()
        })
        .with_extension("json");

    let mut archive = ZipArchive::new(BufReader::new(
        File::open(&args.input).expect("unable to open input file"),
    ))
    .expect("input log file not compressed");
    let mut file = archive.by_index(0).expect("input log file empty");

    let mut log = Log::parse(&mut file).expect("failed to parse EVTC log");
    log.events.sort_by_key(|event| event.time);

    let agent = args.agent.as_deref().map(|arg| {
        log.agents
            .iter()
            .find(|agent| match arg.parse::<u64>() {
                Ok(id) => agent.address == id,
                Err(_) => agent.name[0] == arg,
            })
            .unwrap_or_else(|| panic!("Agent \"{}\" not found", arg))
    });
    let agent_display = agent
        .map(|agent| format!("\"{}\" ({})", agent.name[0], agent.address))
        .unwrap_or_else(|| "all agents".into());

    if let Some(skill_arg) = args.skill {
        let skill = log
            .skills
            .iter()
            .find(|skill| match skill_arg.parse::<u32>() {
                Ok(id) => skill.id == id,
                Err(_) => skill.name == skill_arg,
            })
            .unwrap_or_else(|| panic!("Skill \"{}\" not found", skill_arg));

        println!(
            "Finding casts of skill \"{}\" ({}) for {}",
            skill.name, skill.id, agent_display
        );

        let (casts, hits_without_cast) =
            extract_casts(&log, skill.id, agent.map(|agent| agent.address));

        for info in &hits_without_cast {
            eprintln!(
                "Hit from \"{}\" ({}) at time {} without prior cast",
                agent_name(&log.agents, info.agent)
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

        let output = BufWriter::new(File::create(&out).expect("failed to create output file"));
        serde_json::to_writer_pretty(output, &casts).expect("failed to serialize cast data");
        println!("Saved cast data to \"{}\"", out.display());
    } else if args.position {
        println!("Finding positions of {}", agent_display);

        let positions = extract_positions(&log, agent.map(|agent| agent.address));
        println!("Found {} positions", positions.len());
        if let Some(pos) = positions.first() {
            println!("Spawn at {} {} {}", pos.x, pos.y, pos.z);
        }

        let output = BufWriter::new(File::create(&out).expect("failed to create output file"));
        serde_json::to_writer_pretty(output, &positions)
            .expect("failed to serialize position data");
        println!("Saved position data to \"{}\"", out.display());
    } else {
        println!("No operation specified")
    }
}
