use std::i64;

use crate::constants::*;

pub fn of_seconds_and_adjustment_checked(seconds: i64, nano_adjustment: i64) -> Option<(i64, u32)> {
    let (second_adjustment, nanos) = seconds_and_nanos(nano_adjustment);
    seconds
        .checked_add(second_adjustment)
        .map(|total_seconds| (total_seconds, nanos))
}

pub fn seconds_and_nanos(nanoseconds: i64) -> (i64, u32) {
    let (base_adjustment, base_nanos) = (
        nanoseconds / NANOSECONDS_IN_SECOND,
        nanoseconds % NANOSECONDS_IN_SECOND,
    );

    let (extra_adjustment, final_nanos) = carry_and_nanos(base_nanos);
    (base_adjustment + extra_adjustment, final_nanos)
}

// A second adjustment for when nanoseconds are within 1 step, instead of unbounded.
pub fn carry_and_nanos(nanoseconds: i64) -> (i64, u32) {
    if nanoseconds < 0 {
        (-1, (nanoseconds + NANOSECONDS_IN_SECOND) as u32)
    } else if nanoseconds >= NANOSECONDS_IN_SECOND {
        (1, (nanoseconds - NANOSECONDS_IN_SECOND) as u32)
    } else {
        (0, nanoseconds as u32)
    }
}
