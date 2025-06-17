/// CLI command.
#[derive(Debug, Clone, clap::Subcommand)]
pub enum Command {
    /// Extract all events.
    All,

    /// Extract agents.
    Agents,

    /// Extract skill/buff information.
    Skills {
        /// Id or name of skill to extract data for.
        #[clap(long)]
        skill: Option<String>,
    },

    /// Extract cast & hit data.
    Casts {
        /// Id or name of skill to extract data for.
        #[clap(long)]
        skill: Option<String>,
    },

    /// Extract position data.
    Positions,

    /// Extract visual effect data.
    Effects,

    /// Extract missile (projectile) data.
    Missiles,

    /// Map direct damage hits to weapon sets.
    Hitmap,

    /// Check gear on the recording player.
    Gear,
}

impl Command {
    pub fn suffix(&self) -> Option<&'static str> {
        match self {
            Command::All => None,
            Command::Agents => Some("agents"),
            Command::Skills { .. } => Some("skills"),
            Command::Casts { .. } => Some("casts"),
            Command::Positions => Some("positions"),
            Command::Effects => Some("effects"),
            Command::Missiles => Some("missiles"),
            Command::Hitmap => Some("hitmap"),
            Command::Gear => Some("gear"),
        }
    }
}
