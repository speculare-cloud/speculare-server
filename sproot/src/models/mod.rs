mod balerts;
mod bauth;
mod bserver;
pub mod schema;

pub use balerts::*;
pub use bauth::*;
pub use bserver::*;

/// granularity == the range in which we'll group the data
/// We'll compute the granularity from this equation:
/// f(x) = ((0.00192859 * x) * (1.00694) + 0.298206);
/// which give us ~=:
///  size = 300 => 1
///  size = 900 => 2
///  size = 1800 => 5
///  size = 7200 => 20
///  size = 21600 => 60
/// which means for size = 21600 that we'll get the avg of each 60s intervals
#[inline]
pub fn get_granularity(size: i64) -> u16 {
    assert!(size < 23000000);
    // Casting to u16 is safe as per the check above as u16 max value is 65535 and is not reached unless size is 23000000
    ((0.003 * size as f32) * (0.93) + 0.298206) as u16
}
