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
    pub content_reference: f32,
    pub content_reference_type: u8,
    pub not_npc: bool,
    pub not_player: bool,
    pub is_break: bool,
}

impl From<evtc::BuffFormula> for BuffFormula {
    fn from(event: evtc::BuffFormula) -> Self {
        let evtc::BuffFormula {
            skill_id: _,
            formula,
            attr1,
            attr2,
            param1,
            param2,
            param3,
            trait_src,
            trait_self,
            buff_src,
            buff_self,
            content_reference,
            content_reference_type,
            not_npc,
            not_player,
            is_break,
        } = event;
        Self {
            formula,
            attr1,
            attr2,
            param1,
            param2,
            param3,
            trait_src,
            trait_self,
            buff_src,
            buff_self,
            content_reference,
            content_reference_type,
            not_npc,
            not_player,
            is_break,
        }
    }
}
