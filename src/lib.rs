pub mod agent;
pub mod cast;
pub mod position;
pub mod skill;
mod util;

pub use self::agent::Agent;
pub use self::cast::{extract_casts, Cast, Casts};
pub use self::position::{extract_positions, Position};
pub use self::skill::{extract_skills, Skill, SkillKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BreakbarState {
    Active = 0,
    Recover = 1,
    Immune = 2,
    None = 3,
}
