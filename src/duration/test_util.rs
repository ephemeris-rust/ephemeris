use std::cmp::{max, min};

use proptest::prelude::*;

use crate::{constants::*, Duration};

const MIN_NANOS: i128 = i64::MIN as i128 * NANOSECONDS_IN_SECOND as i128;
const MAX_NANOS: i128 = (i64::MAX as i128 + 1) * NANOSECONDS_IN_SECOND as i128 - 1;

pub(crate) fn arb_duration() -> impl Strategy<Value = Duration> {
    (any::<i64>(), 0..NANOSECONDS_IN_SECOND)
        .prop_map(|(seconds, nanos)| Duration::of_seconds_and_adjustment(seconds, nanos))
}

fn total_nanos(d: Duration) -> i128 {
    d.seconds() as i128 * NANOSECONDS_IN_SECOND as i128 + d.nano() as i128
}

pub(crate) fn arb_duration_range(
    lower: Duration,
    upper: Duration,
) -> impl Strategy<Value = Duration> {
    (total_nanos(lower)..=total_nanos(upper)).prop_map(to_duration)
}

pub(crate) fn arb_duration_remaining_units(
    unit_nanos: i64,
) -> impl Strategy<Value = (Duration, i64)> {
    (
        MIN_NANOS + unit_nanos as i128..=MAX_NANOS - unit_nanos as i128,
        Just(unit_nanos),
    )
        .prop_flat_map(|(nanos, unit_nanos)| {
            let upper = min(
                i64::MAX as i128,
                (MAX_NANOS - max(nanos, 0)) / unit_nanos as i128,
            ) as i64;
            let lower = max(
                i64::MIN as i128,
                (MIN_NANOS - min(nanos, 0)) / unit_nanos as i128,
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

pub(crate) fn arb_duration_overflow_units(
    unit_nanos: i64,
) -> impl Strategy<Value = (Duration, i64)> {
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

pub(crate) fn arb_duration_underflow_units(
    unit_nanos: i64,
) -> impl Strategy<Value = (Duration, i64)> {
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

pub(crate) fn to_duration(nanos: i128) -> Duration {
    let seconds = (nanos / NANOSECONDS_IN_SECOND as i128) as i64;
    let nanos_in_second = (nanos % NANOSECONDS_IN_SECOND as i128) as i64;
    Duration::of_seconds_and_adjustment(seconds, nanos_in_second)
}

pub(crate) fn duration_total() -> impl Strategy<Value = (Duration, Duration, Duration)> {
    (MIN_NANOS..=MAX_NANOS)
        .prop_flat_map(|nanos| {
            let upper = MAX_NANOS - max(nanos, 0);
            let lower = MIN_NANOS - min(nanos, 0);
            (Just(nanos), lower..=upper)
        })
        .prop_map(|(left, right)| {
            (
                to_duration(left),
                to_duration(right),
                to_duration(left + right),
            )
        })
}

pub(crate) fn arb_duration_overflow() -> impl Strategy<Value = (Duration, Duration)> {
    (MAX_NANOS - i64::MAX as i128..=MAX_NANOS)
        .prop_flat_map(|nanos| {
            let lower = MAX_NANOS + 1 - nanos;
            (Just(nanos), lower..=MAX_NANOS)
        })
        .prop_map(|(a, b)| (to_duration(a), to_duration(b)))
}

pub(crate) fn arb_duration_underflow() -> impl Strategy<Value = (Duration, Duration)> {
    (MIN_NANOS..MIN_NANOS - i64::MIN as i128)
        .prop_flat_map(|nanos| {
            let upper = MIN_NANOS - 1 - nanos;
            (Just(nanos), MIN_NANOS..=upper)
        })
        .prop_map(|(a, b)| (to_duration(a), to_duration(b)))
}

pub(crate) fn next(duration: Duration) -> Duration {
    match (duration.seconds(), duration.nano() as i64 + 1) {
        (seconds, NANOSECONDS_IN_SECOND) => Duration::of_seconds(seconds + 1),
        (seconds, nano) => Duration::of_seconds_and_adjustment(seconds, nano),
    }
}
