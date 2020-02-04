use std::cmp::max;
use std::cmp::min;
use std::i128;
use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

const MIN_NANOS: i128 = i64::MIN as i128 * NANOSECONDS_IN_SECOND as i128;
const MAX_NANOS: i128 = (i64::MAX as i128 + 1) * NANOSECONDS_IN_SECOND as i128 - 1;

pub fn arb_duration() -> impl Strategy<Value = Duration> {
    (any::<i64>(), 0..NANOSECONDS_IN_SECOND)
        .prop_map(|(seconds, nanos)| Duration::of_seconds_and_adjustment(seconds, nanos))
}

pub fn arb_duration_remaining_units(unit_nanos: i64) -> impl Strategy<Value = (Duration, i64)> {
    (
        MIN_NANOS + unit_nanos as i128..=MAX_NANOS - unit_nanos as i128,
        Just(unit_nanos),
    )
        .prop_flat_map(|(nanos, unit_nanos)| {
            let upper = min(
                i64::MAX as i128,
                (MAX_NANOS - if nanos > 0 { nanos } else { 0 }) / unit_nanos as i128,
            ) as i64;
            let lower = max(
                i64::MIN as i128,
                (MIN_NANOS - if nanos < 0 { nanos } else { 0 }) / unit_nanos as i128,
            ) as i64;

            let seconds = (nanos / NANOSECONDS_IN_SECOND as i128) as i64;
            let nanos_in_second = (nanos % NANOSECONDS_IN_SECOND as i128) as i64;
            (
                Just(Duration::of_seconds_and_adjustment(
                    seconds,
                    nanos_in_second,
                )),
                lower..=upper,
            )
        })
}

pub fn arb_duration_overflow_units(unit_nanos: i64) -> impl Strategy<Value = (Duration, i64)> {
    (
        max(MIN_NANOS, MAX_NANOS - i64::MAX as i128 * unit_nanos as i128)..=MAX_NANOS,
        Just(unit_nanos),
    )
        .prop_flat_map(|(nanos, unit_nanos)| {
            let lower = min(
                i64::MAX as i128,
                (MAX_NANOS + 1 - nanos) / unit_nanos as i128,
            ) as i64;

            let seconds = (nanos / NANOSECONDS_IN_SECOND as i128) as i64;
            let nanos_in_second = (nanos % NANOSECONDS_IN_SECOND as i128) as i64;

            (
                Just(Duration::of_seconds_and_adjustment(
                    seconds,
                    nanos_in_second,
                )),
                lower..=i64::MAX,
            )
        })
}

pub fn arb_duration_underflow_units(unit_nanos: i64) -> impl Strategy<Value = (Duration, i64)> {
    (
        MIN_NANOS..min(MAX_NANOS, MIN_NANOS - i64::MIN as i128 * unit_nanos as i128),
        Just(unit_nanos),
    )
        .prop_flat_map(|(nanos, unit_nanos)| {
            let upper = max(
                i64::MIN as i128,
                (MIN_NANOS - 1 - nanos) / unit_nanos as i128,
            ) as i64;

            let seconds = (nanos / NANOSECONDS_IN_SECOND as i128) as i64;
            let nanos_in_second = (nanos % NANOSECONDS_IN_SECOND as i128) as i64;

            (
                Just(Duration::of_seconds_and_adjustment(
                    seconds,
                    nanos_in_second,
                )),
                i64::MIN..upper,
            )
        })
}

pub fn next(duration: Duration) -> Duration {
    match (duration.seconds(), duration.nano() as i64 + 1) {
        (seconds, NANOSECONDS_IN_SECOND) => Duration::of_seconds(seconds + 1),
        (seconds, nano) => Duration::of_seconds_and_adjustment(seconds, nano),
    }
}
