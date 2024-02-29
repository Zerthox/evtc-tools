use evtc_parse::{self as evtc, Event, StateChange};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponMap {
    pub swaps: Vec<Swap>,
}

impl WeaponMap {
    pub fn new<'a>(events: impl IntoIterator<Item = &'a Event>, agent: u64) -> Self {
        Self {
            swaps: events
                .into_iter()
                .filter(|event| {
                    event.src_agent == agent && event.get_statechange() == StateChange::WeaponSwap
                })
                .filter_map(|event| {
                    let set = evtc::weapon::WeaponSet::from(event.dst_agent);
                    WeaponSet::try_from(set).ok().map(|set| Swap {
                        time: event.time,
                        set,
                    })
                })
                .collect(),
        }
    }

    pub fn set_at(&self, time: u64) -> Option<WeaponSet> {
        match self.swaps.binary_search_by_key(&time, |swap| swap.time) {
            Ok(index) => self.swaps.get(index).map(|swap| swap.set),
            Err(0) => None,
            Err(index) => self.swaps.get(index - 1).map(|swap| swap.set),
        }
    }

    pub fn sets(&self) -> impl Iterator<Item = WeaponSet> {
        self.swaps
            .iter()
            .map(|swap| swap.set)
            .collect::<HashSet<_>>()
            .into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swap {
    pub time: u64,
    pub set: WeaponSet,
}

#[derive(
    Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum WeaponSet {
    #[default]
    Initial,
    Water1,
    Water2,
    Land1,
    Land2,
}

impl TryFrom<evtc::weapon::WeaponSet> for WeaponSet {
    type Error = evtc::weapon::WeaponSet;

    fn try_from(set: evtc::weapon::WeaponSet) -> Result<Self, Self::Error> {
        match set {
            evtc::weapon::WeaponSet::Water1 => Ok(Self::Water1),
            evtc::weapon::WeaponSet::Water2 => Ok(Self::Water2),
            evtc::weapon::WeaponSet::Land1 => Ok(Self::Land1),
            evtc::weapon::WeaponSet::Land2 => Ok(Self::Land2),
            set => Err(set),
        }
    }
}
