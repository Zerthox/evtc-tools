pub mod agent;
pub mod cast;
pub mod effect;
pub mod gear;
pub mod hit;
pub mod hit_map;
pub mod missile;
pub mod position;
pub mod skills;
pub mod time;
pub mod util;
pub mod weapons;

pub use self::agent::Agent;
pub use self::cast::{extract_casts, Cast, Casts};
pub use self::effect::{extract_effects, Effect};
pub use self::gear::{extract_gear, GearInfo, GearItem, Rune, Sigil};
pub use self::hit::{Hit, HitWithSkill};
pub use self::missile::Missile;
pub use self::position::{extract_positions, Position};
pub use self::skills::{extract_skills, SkillIdName, SkillKind, SkillWithInfo};
pub use self::time::Time;
pub use self::weapons::{WeaponMap, WeaponSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BreakbarState {
    Active = 0,
    Recover = 1,
    Immune = 2,
    None = 3,
}
