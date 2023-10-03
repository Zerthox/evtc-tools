use arcdps_parse::{self as arcdps, CombatEvent, StateChange};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponMap {
    pub swaps: Vec<Swap>,
}

impl WeaponMap {
    pub fn new<'a>(events: impl IntoIterator<Item = &'a CombatEvent>, agent: u64) -> Self {
        Self {
            swaps: events
                .into_iter()
                .filter(|event| {
                    event.src_agent == agent && event.is_statechange == StateChange::WeaponSwap
                })
                .filter_map(|event| {
                    arcdps::WeaponSet::try_from(event.dst_agent)
                        .ok()
                        .and_then(|set| WeaponSet::try_from(set).ok())
                        .map(|set| Swap {
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

impl TryFrom<arcdps::WeaponSet> for WeaponSet {
    type Error = arcdps::WeaponSet;

    fn try_from(set: arcdps::WeaponSet) -> Result<Self, Self::Error> {
        match set {
            arcdps::WeaponSet::Water1 => Ok(Self::Water1),
            arcdps::WeaponSet::Water2 => Ok(Self::Water2),
            arcdps::WeaponSet::Land1 => Ok(Self::Land1),
            arcdps::WeaponSet::Land2 => Ok(Self::Land2),
            set => Err(set),
        }
    }
}
