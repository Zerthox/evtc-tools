pub const TICK: u64 = 40;

/// Converts a milliseconds timestamp to ticks.
pub fn to_tick_rounded(time: i32) -> i32 {
    (time as f64 / TICK as f64).round() as i32
}
