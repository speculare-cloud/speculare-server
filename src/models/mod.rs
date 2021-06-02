pub mod schema;

mod cpustats;
pub use cpustats::*;

mod cputimes;
pub use cputimes::*;

mod disks;
pub use disks::*;

mod hosts;
pub use hosts::*;

mod http_models;
pub use http_models::*;

mod iocounters;
pub use iocounters::*;

mod iostats;
pub use iostats::*;

mod loadavg;
pub use loadavg::*;

mod memory;
pub use memory::*;

mod swap;
pub use swap::*;

/// granularity == the range in which we'll group the data
/// We'll compute the granularity from this equation:
/// f(x) = ((0.00192859 * x) * (1.00694) + 0.298206);
/// which give us ~=:
/// size = 300 => 1
/// size = 900 => 2
/// size = 1800 => 4
/// size = 7200 => 15
/// size = 21600 => 45
#[inline]
pub fn get_granularity(size: i64) -> u16 {
    assert!(size < 23000000);
    // Casting to u16 is safe as per the check above as u16 max value is 65535 and is not reached unless size is 23000000
    ((0.003 * size as f32) * (0.93) + 0.298206) as u16
}

#[inline]
pub fn get_query_range_values(granularity: u16) -> (i64, i64, i64) {
    if granularity > 60 {
        ((granularity / 60) as i64, (granularity % 60) as i64, 0i64)
    } else {
        (1i64, 0i64, granularity as i64)
    }
}
