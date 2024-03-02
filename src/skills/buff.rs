use evtc_parse::event as evtc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffInfo {
    pub category: u8,
    pub stacking_type: u8,
    pub max_stacks: u16,
    pub duration_cap: u32,
    pub invulnerable: bool,
    pub invert: bool,
    pub resistance: bool,
    pub formulas: Vec<BuffFormula>,
}

impl BuffInfo {
    pub fn new(
        info: evtc::BuffInfo,
        formulas: impl IntoIterator<Item = evtc::BuffFormula>,
    ) -> Self {
        Self {
            category: info.category,
            stacking_type: info.stacking_type,
            max_stacks: info.max_stacks,
            duration_cap: info.duration_cap,
            invulnerable: info.invulnerable,
            invert: info.invert,
            resistance: info.resistance,
            formulas: formulas.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuffFormula {
    pub formula: u32,
    pub attr1: u32,
    pub attr2: u32,
    pub param1: f32,
    pub param2: f32,
    pub param3: f32,
    pub trait_src: u32,
    pub trait_self: u32,
    pub buff_src: u32,
    pub buff_self: u32,
    pub not_npc: bool,
    pub not_player: bool,
    pub is_break: bool,
    pub value: u32,
    pub value_type: u8,
}

impl From<evtc::BuffFormula> for BuffFormula {
    fn from(event: evtc::BuffFormula) -> Self {
        Self {
            formula: event.formula,
            attr1: event.attr1,
            attr2: event.attr2,
            param1: event.param1,
            param2: event.param2,
            param3: event.param3,
            trait_src: event.trait_src,
            trait_self: event.trait_self,
            buff_src: event.buff_src,
            buff_self: event.buff_self,
            not_npc: event.not_npc,
            not_player: event.not_player,
            is_break: event.is_break,
            value: event.value,
            value_type: event.value_type,
        }
    }
}
