use evtc_parse::content::{ContentInfo, ContentType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectInfo {
    pub content_id: u32,
    pub guid: String,
    pub guid_misinterpret: String,
    pub effect_type: u16,
    pub default_duration: f32,
}

impl EffectInfo {
    pub fn new(event: ContentInfo) -> Option<Self> {
        match event.content_type {
            ContentType::Effect {
                effect_type,
                default_duration,
            } => Some(Self {
                content_id: event.content_id,
                guid: event.guid_string(),
                guid_misinterpret: event.guid_string(),
                effect_type,
                default_duration,
            }),
            _ => None,
        }
    }
}
