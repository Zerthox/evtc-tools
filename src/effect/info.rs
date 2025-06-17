use evtc_parse::content::{ContentInfo, ContentType, GuidExt};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectInfo {
    pub content_id: u32,
    pub guid: String,
    pub guid_bytes: String,
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
                guid: event.guid.format_hyphenated(),
                guid_bytes: format!(
                    "{:0>32X}",
                    u128::from_be_bytes(unsafe { event.guid.misinterpret() })
                ),
                effect_type,
                default_duration,
            }),
            _ => None,
        }
    }
}
