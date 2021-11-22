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

prop_compose! {
    #[allow(clippy::zero_prefixed_literal)]
    fn multiply_specific_data()
        (data in proptest::sample::select(
            vec![
                (Duration::of_seconds_and_adjustment(-4, 666666667), -3, Duration::of_seconds_and_adjustment(9, 999999999)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), -2, Duration::of_seconds_and_adjustment(6, 666666666)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), -1, Duration::of_seconds_and_adjustment(3, 333333333)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), 1, Duration::of_seconds_and_adjustment(-4, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), 2, Duration::of_seconds_and_adjustment(-7, 333333334)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), 3, Duration::of_seconds_and_adjustment(-10, 000000001)),

                (Duration::of_seconds_and_adjustment(-3, 0), -3, Duration::of_seconds_and_adjustment(9, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), -2, Duration::of_seconds_and_adjustment(6, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), -1, Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), 1, Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), 2, Duration::of_seconds_and_adjustment(-6, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), 3, Duration::of_seconds_and_adjustment(-9, 0)),

                (Duration::of_seconds_and_adjustment(-2, 0), -3, Duration::of_seconds_and_adjustment(6, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), -2, Duration::of_seconds_and_adjustment(4, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), -1, Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), 1, Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), 2, Duration::of_seconds_and_adjustment(-4, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), 3, Duration::of_seconds_and_adjustment(-6, 0)),

                (Duration::of_seconds_and_adjustment(-1, 0), -3, Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), -2, Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), -1, Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), 1, Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), 2, Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), 3, Duration::of_seconds_and_adjustment(-3, 0)),

                (Duration::of_seconds_and_adjustment(-1, 500000000), -3, Duration::of_seconds_and_adjustment(1, 500000000)),
                (Duration::of_seconds_and_adjustment(-1, 500000000), -2, Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(-1, 500000000), -1, Duration::of_seconds_and_adjustment(0, 500000000)),
                (Duration::of_seconds_and_adjustment(-1, 500000000), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-1, 500000000), 1, Duration::of_seconds_and_adjustment(-1, 500000000)),
                (Duration::of_seconds_and_adjustment(-1, 500000000), 2, Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(-1, 500000000), 3, Duration::of_seconds_and_adjustment(-2, 500000000)),

                (Duration::of_seconds_and_adjustment(0, 0), -3, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), -2, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), -1, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), 1, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), 2, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), 3, Duration::of_seconds_and_adjustment(0, 0)),

                (Duration::of_seconds_and_adjustment(0, 500000000), -3, Duration::of_seconds_and_adjustment(-2, 500000000)),
                (Duration::of_seconds_and_adjustment(0, 500000000), -2, Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(0, 500000000), -1, Duration::of_seconds_and_adjustment(-1, 500000000)),
                (Duration::of_seconds_and_adjustment(0, 500000000), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 500000000), 1, Duration::of_seconds_and_adjustment(0, 500000000)),
                (Duration::of_seconds_and_adjustment(0, 500000000), 2, Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(0, 500000000), 3, Duration::of_seconds_and_adjustment(1, 500000000)),

                (Duration::of_seconds_and_adjustment(1, 0), -3, Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), -2, Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), -1, Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), 1, Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), 2, Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), 3, Duration::of_seconds_and_adjustment(3, 0)),

                (Duration::of_seconds_and_adjustment(2, 0), -3, Duration::of_seconds_and_adjustment(-6, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), -2, Duration::of_seconds_and_adjustment(-4, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), -1, Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), 1, Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), 2, Duration::of_seconds_and_adjustment(4, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), 3, Duration::of_seconds_and_adjustment(6, 0)),

                (Duration::of_seconds_and_adjustment(3, 0), -3, Duration::of_seconds_and_adjustment(-9, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), -2, Duration::of_seconds_and_adjustment(-6, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), -1, Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), 1, Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), 2, Duration::of_seconds_and_adjustment(6, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), 3, Duration::of_seconds_and_adjustment(9, 0)),

                (Duration::of_seconds_and_adjustment(3, 333333333), -3, Duration::of_seconds_and_adjustment(-10, 000000001)),
                (Duration::of_seconds_and_adjustment(3, 333333333), -2, Duration::of_seconds_and_adjustment(-7, 333333334)),
                (Duration::of_seconds_and_adjustment(3, 333333333), -1, Duration::of_seconds_and_adjustment(-4, 666666667)),
                (Duration::of_seconds_and_adjustment(3, 333333333), 0, Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(3, 333333333), 1, Duration::of_seconds_and_adjustment(3, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), 2, Duration::of_seconds_and_adjustment(6, 666666666)),
                (Duration::of_seconds_and_adjustment(3, 333333333), 3, Duration::of_seconds_and_adjustment(9, 999999999)),

                (Duration::of_seconds(1), i64::MAX, Duration::of_seconds(i64::MAX)),
                (Duration::of_seconds(1), i64::MIN, Duration::MIN)
            ]
        )) -> (Duration, i64, Duration)
        {
            data
        }
}

proptest! {
    #[test]
    #[allow(clippy::erasing_op)]
    fn multiply_unit(duration in arb_duration()) {
        prop_assert_eq!(Duration::ZERO, duration * 0);
    }

    #[test]
    #[allow(clippy::identity_op)]
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
    fn multiply_overflow((duration, scalar) in arb_duration_overflow()) {
        expect_panic("duration multiplication would overflow", || duration * scalar)?;
    }

    #[test]
    fn multiply_underflow((duration, scalar) in arb_duration_underflow()) {
        expect_panic("duration multiplication would overflow", || duration * scalar)?;
    }

    #[test]
    fn multiply_specific((duration, factor, expected) in multiply_specific_data()) {
        prop_assert_eq!(duration * factor, expected);
    }


    #[test]
    fn multiply_assign_unit(duration in arb_duration()) {
        let mut other = duration;
        other *= 0;

        prop_assert_eq!(Duration::ZERO, other);
    }

    #[test]
    fn multiply_assign_identity(duration in arb_duration()) {
        let mut other = duration;
        other *= 1;

        prop_assert_eq!(duration, other);
    }

    #[test]
    fn multiply_assign_associative(
        duration in arb_duration_range(Duration::of_seconds(i32::MIN as i64 - 1), Duration::of_seconds(i32::MAX as i64 + 1)),
        a in prop::num::i16::ANY.prop_map(|i| i as i64),
        b in prop::num::i16::ANY.prop_map(|i| i as i64)) {

        let mut left_a = duration;
        left_a *= a;
        left_a *= b;

        let mut left_b = duration;
        left_b *= a * b;

        prop_assert_eq!(left_a, left_b);
    }

    #[test]
    fn multiply_assign_distributive(
        left in arb_duration_range(Duration::of_seconds(i32::MIN as i64 - 1), Duration::of_seconds(i32::MAX as i64 + 1)),
        right in arb_duration_range(Duration::of_seconds(i32::MIN as i64 - 1), Duration::of_seconds(i32::MAX as i64 + 1)),
        factor in prop::num::i32::ANY.prop_map(|i| i as i64)) {

        let mut added = left + right;
        added *= factor;

        let mut multiplied_left = left;
        multiplied_left *= factor;

        let mut multiplied_right = right;
        multiplied_right *= factor;

        prop_assert_eq!(added, multiplied_left + multiplied_right);
    }

    #[test]
    fn multiply_assign_overflow((duration, scalar) in arb_duration_overflow()) {
        expect_panic("duration multiplication would overflow", || {
            let mut duration_copy = duration;
            duration_copy *= scalar;
        })?;
    }

    #[test]
    fn multiply_assign_underflow((duration, scalar) in arb_duration_underflow()) {
        expect_panic("duration multiplication would overflow", || {
            let mut duration_copy = duration;
            duration_copy *= scalar;
        })?;
    }

    #[test]
    fn multiply_assign_specific((duration, factor, expected) in multiply_specific_data()) {
        let mut duration_copy = duration;
        duration_copy *= factor;
        prop_assert_eq!(duration_copy, expected);
    }
}
