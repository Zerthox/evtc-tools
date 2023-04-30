use arcdps_parse::{parse_file, Agent, CombatEvent, Log};
use clap::Parser;
use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

/// CLI interface.
#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Command.
    #[command(subcommand)]
    pub command: Command,

    #[clap(flatten)]
    pub args: Args,
}

/// Arguments.
#[derive(Debug, Clone, clap::Args)]
pub struct Args {
    /// Path to input file.
    #[clap(global = true, default_value_t)]
    pub input: String,

    /// Path to output file, defaults to input filename.
    #[clap(global = true)]
    pub output: Option<PathBuf>,

    /// Id or name of source agent to filter data for.
    #[clap(short, long, global = true)]
    pub agent: Option<String>,

    /// Id or name of destination agent to filter data for.
    #[clap(short, long, global = true)]
    pub target: Option<String>,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    /// Extract all events.
    All,

    /// Extract cast & hit data.
    Cast {
        /// Id or name of skill to extract data for.
        #[clap(long)]
        skill: Option<String>,
    },

    /// Extract skill/buff information.
    Skill {
        /// Id or name of skill to extract data for.
        #[clap(long)]
        skill: Option<String>,
    },

    /// Extract position data.
    Position,

    ///Extract effect data.
    Effect,
}

impl Args {
    pub fn open_log(&self) -> Log {
        parse_file(&self.input).expect("failed to parse EVTC log")
    }

    fn create_filter(log: &Log, arg: &Option<String>, kind: &str) -> Filter {
        arg.as_deref()
            .map(|arg| {
                let filter = Filter::parse(log, arg);
                let agent = filter.agent(log);
                match (&filter, agent) {
                    (_, Some(agent)) => {
                        println!(
                            "Agent {} filter: \"{}\" ({})",
                            kind, agent.name[0], agent.address
                        )
                    }
                    (Filter::ArcId(id), None) => {
                        println!("Agent {} filter: unknown agent id {}", kind, id)
                    }
                    (Filter::InstId(id), None) => {
                        println!("Agent {} filter: unknown agent instance id {}", kind, id)
                    }
                    _ => {}
                }
                filter
            })
            .unwrap_or_default()
    }

    pub fn filter_log<'a>(&self, log: &'a Log) -> impl Iterator<Item = &'a CombatEvent> {
        let src = Self::create_filter(log, &self.agent, "source");
        let dst = Self::create_filter(log, &self.target, "dest");

        log.events
            .iter()
            .filter(move |event| src.filter(event) && dst.filter(event))
    }

    pub fn write_output<T>(&self, value: &T)
    where
        T: ?Sized + serde::Serialize,
    {
        let out = self
            .output
            .clone()
            .unwrap_or_else(|| {
                Path::new(&self.input)
                    .file_name()
                    .expect("input path is no file")
                    .into()
            })
            .with_extension("json");

        let output = BufWriter::new(File::create(&out).expect("failed to create output file"));
        serde_json::to_writer_pretty(output, value).expect("failed to serialize data");
        println!("Saved data to \"{}\"", out.display());
    }
}

// TODO: filter all by name not just first id
#[derive(Debug, Default, Clone)]
enum Filter {
    #[default]
    None,
    ArcId(u64),
    InstId(u16),
}

impl Filter {
    fn parse(log: &Log, value: &str) -> Self {
        if let Some(string) = value.strip_prefix("inst:") {
            let id = string.parse().expect("invalid instance id");
            Self::InstId(id)
        } else {
            match value.parse::<u64>() {
                Ok(id) => Self::ArcId(id),
                Err(_) => log
                    .agents
                    .iter()
                    .find(|agent| agent.name[0] == value)
                    .map(|agent| Self::ArcId(agent.address))
                    .unwrap_or_else(|| panic!("failed to find agent \"{}\"", value)),
            }
        }
    }

    fn agent<'a>(&self, log: &'a Log) -> Option<&'a Agent> {
        match *self {
            Self::None => None,
            Self::ArcId(id) => log.agent(id),
            Self::InstId(id) => log
                .events
                .iter()
                .find(|event| event.src_instance_id == id)
                .and_then(|event| log.agent(event.src_agent)),
        }
    }

    fn filter(&self, event: &CombatEvent) -> bool {
        match *self {
            Self::None => true,
            Self::ArcId(id) => event.src_agent == id,
            Self::InstId(id) => event.src_instance_id == id,
        }
    }
}
