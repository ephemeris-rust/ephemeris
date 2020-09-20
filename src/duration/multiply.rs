use std::i128;
use std::i64;

use proptest::prelude::*;
use proptest::sample::*;

use crate::constants::*;

use crate::Duration;

use crate::assert::expect_panic;

use crate::duration::test_util::arb_duration;
use crate::duration::test_util::arb_duration_range;
use crate::duration::test_util::to_duration;

const MIN_NANOS: i128 = i64::MIN as i128 * NANOSECONDS_IN_SECOND as i128;
const MAX_NANOS: i128 = (i64::MAX as i128 + 1) * NANOSECONDS_IN_SECOND as i128 - 1;

pub fn arb_duration_overflow() -> impl Strategy<Value = (Duration, i64)> {
    (MAX_NANOS / i64::MAX as i128..=MAX_NANOS)
        .prop_flat_map(|nanos| {
            let lower = (MAX_NANOS / nanos + 1) as i64;
            (Just(nanos), lower..=i64::MAX, select(vec![-1i64, 1i64]))
        })
        .prop_map(|(nanos, factor, sign)| (to_duration(nanos * sign as i128), factor * sign))
}

pub fn arb_duration_underflow() -> impl Strategy<Value = (Duration, i64)> {
    (MIN_NANOS..=MIN_NANOS / i64::MAX as i128)
        .prop_flat_map(|nanos| {
            let lower = (MIN_NANOS / nanos - 1) as i64;
            (Just(nanos), lower..=i64::MAX, select(vec![-1i64, 1i64]))
        })
        .prop_map(|(nanos, factor, sign)| (to_duration(nanos * sign as i128), factor * sign))
}

proptest! {
    #[test]
    fn multiply_unit(duration in arb_duration()) {
        prop_assert_eq!(Duration::ZERO, duration * 0);
    }

    #[test]
    fn multiply_identity(duration in arb_duration()) {
        prop_assert_eq!(duration, duration * 1);
    }

    #[test]
    fn multiply_associative(
        duration in arb_duration_range(Duration::of_seconds(i32::MIN as i64 - 1), Duration::of_seconds(i32::MAX as i64 + 1)), 
        a in prop::num::i16::ANY.prop_map(|i| i as i64), 
        b in prop::num::i16::ANY.prop_map(|i| i as i64)) {
        let left = (duration * a) * b;
        let right = duration * (a * b);
        prop_assert_eq!(left, right);
    }

    #[test]
    fn multiply_distributive(
        left in arb_duration_range(Duration::of_seconds(i32::MIN as i64 - 1), Duration::of_seconds(i32::MAX as i64 + 1)), 
        right in arb_duration_range(Duration::of_seconds(i32::MIN as i64 - 1), Duration::of_seconds(i32::MAX as i64 + 1)), 
        factor in prop::num::i32::ANY.prop_map(|i| i as i64)) {
        let added = (left + right) * factor;
        let multiplied = (left * factor) + (right * factor);
        prop_assert_eq!(added, multiplied);
    }

    #[test]
    fn add_overflow((duration, scalar) in arb_duration_overflow()) {
        expect_panic("duration multiplication would overflow", || duration * scalar)?;
    }

    #[test]
    fn add_underflow((duration, scalar) in arb_duration_underflow()) {
        expect_panic("duration multiplication would overflow", || duration * scalar)?;
    }
}
