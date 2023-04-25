pub const TICK: u64 = 40;

/// Converts a milliseconds timestamp to ticks.
pub fn to_tick_rounded(time: u64) -> u64 {
    (time as f64 / TICK as f64).round() as u64
}
