use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearBuff {
    pub time: i32,
    pub id: u32,
    pub log_name: String,
}

impl PartialEq for GearBuff {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for GearBuff {}

impl Hash for GearBuff {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GearItem<T> {
    #[serde(flatten)]
    pub buff: GearBuff,
    pub item: T,
}

impl<T> GearItem<T>
where
    T: TryFrom<u32>,
{
    pub const fn new(buff: GearBuff, item: T) -> Self {
        Self { buff, item }
    }
}

impl<T> PartialEq for GearItem<T> {
    fn eq(&self, other: &Self) -> bool {
        self.buff.eq(&other.buff)
    }
}

impl<T> Eq for GearItem<T> {}

impl<T> Hash for GearItem<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.buff.hash(state)
    }
}
