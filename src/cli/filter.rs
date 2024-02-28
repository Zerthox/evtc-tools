use evtc_parse::{Agent, Event, Log};

// TODO: filter all by name, not just pick first matching id
#[derive(Debug, Default, Clone)]
pub enum AgentFilter {
    #[default]
    None,
    ArcId(u64),
    InstId(u16),
}

impl AgentFilter {
    pub fn parse(log: &Log, value: &str) -> Self {
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
                    .map(|agent| Self::ArcId(agent.id))
                    .unwrap_or_else(|| panic!("failed to find agent \"{}\"", value)),
            }
        }
    }

    pub fn agent<'a>(&self, log: &'a Log) -> Option<&'a Agent> {
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

    pub fn filter_src(&self, event: &Event) -> bool {
        match *self {
            Self::None => true,
            Self::ArcId(id) => event.src_agent == id,
            Self::InstId(id) => event.src_instance_id == id,
        }
    }

    pub fn filter_dst(&self, event: &Event) -> bool {
        match *self {
            Self::None => true,
            Self::ArcId(id) => event.dst_agent == id,
            Self::InstId(id) => event.dst_instance_id == id,
        }
    }
}
