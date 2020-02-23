use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

use crate::assert::expect_panic;

use crate::duration::test_util::*;

proptest! {
    #[test]
    fn plus_zero_days(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_days(0));
    }

    #[test]
    fn identity_duration_plus_days(days in i64::MIN / SECONDS_IN_DAY..i64::MAX / SECONDS_IN_DAY) {
        let added = next(Duration::ZERO.plus_days(days));
        let other = next(Duration::ZERO).plus_days(days);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_days((duration, days) in arb_duration_remaining_units(NANOSECONDS_IN_DAY)) {
        let added = next(duration.plus_days(days));
        let other = next(duration).plus_days(days);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_days_overflow((duration, days) in arb_duration_overflow_units(NANOSECONDS_IN_DAY)) {
        expect_panic("addition of days overflowed", || duration.plus_days(days))?;
    }

    #[test]
    fn plus_days_underflow((duration, days) in arb_duration_underflow_units(NANOSECONDS_IN_DAY)) {
        expect_panic("addition of days overflowed", || duration.plus_days(days))?;
    }

    #[test]
    fn plus_days_specific((base, amount, expected)
        in prop_oneof!(
            Just((Duration::of_days(0), 0, 0)),
            Just((Duration::of_days(0), 1, 1)),
            Just((Duration::of_days(0), -1, -1)),
            Just((Duration::of_days(i64::MAX / SECONDS_IN_DAY), 0, i64::MAX / SECONDS_IN_DAY)),
            Just((Duration::of_days(i64::MIN / SECONDS_IN_DAY), 0, i64::MIN / SECONDS_IN_DAY)),
            Just((Duration::of_days(1), 0, 1)),
            Just((Duration::of_days(1), 1, 2)),
            Just((Duration::of_days(1), -1, 0)),
            Just((Duration::of_days(1), i64::MIN / SECONDS_IN_DAY, i64::MIN / SECONDS_IN_DAY + 1)),
            Just((Duration::of_days(-1), 0, -1)),
            Just((Duration::of_days(-1), 1, 0)),
            Just((Duration::of_days(-1), -1, -2)),
            Just((Duration::of_days(-1), i64::MAX / SECONDS_IN_DAY, i64::MAX / SECONDS_IN_DAY - 1)),
        )) {
        let added = base.plus_days(amount);
        prop_assert_eq!(added.to_days(), expected);
    }

    #[test]
    fn plus_zero_hours(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_hours(0));
    }

    #[test]
    fn identity_duration_plus_hours(hours in i64::MIN / SECONDS_IN_HOUR..i64::MAX / SECONDS_IN_HOUR) {
        let added = next(Duration::ZERO.plus_hours(hours));
        let other = next(Duration::ZERO).plus_hours(hours);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_hours((duration, hours) in arb_duration_remaining_units(NANOSECONDS_IN_HOUR)) {
        let added = next(duration.plus_hours(hours));
        let other = next(duration).plus_hours(hours);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_hours_overflow((duration, hours) in arb_duration_overflow_units(NANOSECONDS_IN_HOUR)) {
        expect_panic("addition of hours overflowed", || duration.plus_hours(hours))?;
    }

    #[test]
    fn plus_hours_underflow((duration, hours) in arb_duration_underflow_units(NANOSECONDS_IN_HOUR)) {
        expect_panic("addition of hours overflowed", || duration.plus_hours(hours))?;
    }

    #[test]
    fn plus_hours_specific((base, amount, expected)
        in prop_oneof!(
            Just((Duration::of_hours(0), 0, 0)),
            Just((Duration::of_hours(0), 1, 1)),
            Just((Duration::of_hours(0), -1, -1)),
            Just((Duration::of_hours(i64::MAX / SECONDS_IN_HOUR), 0, i64::MAX / SECONDS_IN_HOUR)),
            Just((Duration::of_hours(i64::MIN / SECONDS_IN_HOUR), 0, i64::MIN / SECONDS_IN_HOUR)),
            Just((Duration::of_hours(1), 0, 1)),
            Just((Duration::of_hours(1), 1, 2)),
            Just((Duration::of_hours(1), -1, 0)),
            Just((Duration::of_hours(1), i64::MIN / SECONDS_IN_HOUR, i64::MIN / SECONDS_IN_HOUR + 1)),
            Just((Duration::of_hours(-1), 0, -1)),
            Just((Duration::of_hours(-1), 1, 0)),
            Just((Duration::of_hours(-1), -1, -2)),
            Just((Duration::of_hours(-1), i64::MAX / SECONDS_IN_HOUR, i64::MAX / SECONDS_IN_HOUR - 1)),
        )) {
        let added = base.plus_hours(amount);
        prop_assert_eq!(added.to_hours(), expected);
    }

    #[test]
    fn plus_zero_minutes(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_minutes(0));
    }

    #[test]
    fn identity_duration_plus_minutes(minutes in i64::MIN / SECONDS_IN_MINUTE..i64::MAX / SECONDS_IN_MINUTE) {
        let added = next(Duration::ZERO.plus_minutes(minutes));
        let other = next(Duration::ZERO).plus_minutes(minutes);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_minutes((duration, minutes) in arb_duration_remaining_units(NANOSECONDS_IN_MINUTE)) {
        let added = next(duration.plus_minutes(minutes));
        let other = next(duration).plus_minutes(minutes);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_minutes_overflow((duration, minutes) in arb_duration_overflow_units(NANOSECONDS_IN_MINUTE)) {
        expect_panic("addition of minutes overflowed", || duration.plus_minutes(minutes))?;
    }

    #[test]
    fn plus_minutes_underflow((duration, minutes) in arb_duration_underflow_units(NANOSECONDS_IN_MINUTE)) {
        expect_panic("addition of minutes overflowed", || duration.plus_minutes(minutes))?;
    }

    #[test]
    fn plus_minutes_specific((base, amount, expected)
        in prop_oneof!(
            Just((Duration::of_minutes(0), 0, 0)),
            Just((Duration::of_minutes(0), 1, 1)),
            Just((Duration::of_minutes(0), -1, -1)),
            Just((Duration::of_minutes(i64::MAX / SECONDS_IN_MINUTE), 0, i64::MAX / SECONDS_IN_MINUTE)),
            Just((Duration::of_minutes(i64::MIN / SECONDS_IN_MINUTE), 0, i64::MIN / SECONDS_IN_MINUTE)),
            Just((Duration::of_minutes(1), 0, 1)),
            Just((Duration::of_minutes(1), 1, 2)),
            Just((Duration::of_minutes(1), -1, 0)),
            Just((Duration::of_minutes(1), i64::MIN / SECONDS_IN_MINUTE, i64::MIN / SECONDS_IN_MINUTE + 1)),
            Just((Duration::of_minutes(-1), 0, -1)),
            Just((Duration::of_minutes(-1), 1, 0)),
            Just((Duration::of_minutes(-1), -1, -2)),
            Just((Duration::of_minutes(-1), i64::MAX / SECONDS_IN_MINUTE, i64::MAX / SECONDS_IN_MINUTE - 1)),
        )) {
        let added = base.plus_minutes(amount);
        prop_assert_eq!(added.to_minutes(), expected);
    }

    #[test]
    fn plus_zero_seconds(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_seconds(0));
    }

    #[test]
    fn identity_duration_plus_seconds(seconds in i64::MIN..i64::MAX) {
        let added = next(Duration::ZERO.plus_seconds(seconds));
        let other = next(Duration::ZERO).plus_seconds(seconds);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_seconds((duration, seconds) in arb_duration_remaining_units(NANOSECONDS_IN_SECOND)) {
        let added = next(duration.plus_seconds(seconds));
        let other = next(duration).plus_seconds(seconds);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_seconds_overflow((duration, seconds) in arb_duration_overflow_units(NANOSECONDS_IN_SECOND)) {
        expect_panic("addition of seconds overflowed", || duration.plus_seconds(seconds))?;
    }

    #[test]
    fn plus_seconds_underflow((duration, seconds) in arb_duration_underflow_units(NANOSECONDS_IN_SECOND)) {
        expect_panic("addition of seconds overflowed", || duration.plus_seconds(seconds))?;
    }

    #[test]
    fn plus_seconds_specific((base, amount, expected_seconds, expected_nano)
        in prop_oneof!(
            Just((Duration::of_seconds(0), 0, 0, 0)),
            Just((Duration::of_seconds(0), 1, 1, 0)),
            Just((Duration::of_seconds(0), -1, -1, 0)),
            Just((Duration::of_seconds(i64::MAX), 0, i64::MAX, 0)),
            Just((Duration::of_seconds(i64::MIN), 0, i64::MIN, 0)),
            Just((Duration::of_seconds(1), 0, 1, 0)),
            Just((Duration::of_seconds(1), 1, 2, 0)),
            Just((Duration::of_seconds(1), -1, 0, 0)),
            Just((Duration::of_seconds(1), i64::MAX - 1, i64::MAX, 0)),
            Just((Duration::of_seconds(1), i64::MIN, i64::MIN + 1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 0, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), i64::MAX - 1, i64::MAX, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), i64::MIN, i64::MIN + 1, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), 0, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), 1, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), -1, -2, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), i64::MAX, i64::MAX - 1, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), i64::MIN + 1, i64::MIN, 1)),
        )) {
        let added = base.plus_seconds(amount);
        prop_assert_eq!(added.seconds(), expected_seconds);
        prop_assert_eq!(added.nano(), expected_nano);
    }

    #[test]
    fn plus_zero_millis(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_millis(0));
    }

    #[test]
    fn identity_duration_plus_millis(millis in i64::MIN..=i64::MAX) {
        let added = next(Duration::ZERO.plus_millis(millis));
        let other = next(Duration::ZERO).plus_millis(millis);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_millis((duration, millis) in arb_duration_remaining_units(NANOSECONDS_IN_MILLISECOND)) {
        let added = next(duration.plus_millis(millis));
        let other = next(duration).plus_millis(millis);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_millis_overflow((duration, millis) in arb_duration_overflow_units(NANOSECONDS_IN_MILLISECOND)) {
        expect_panic("addition of millis overflowed", || duration.plus_millis(millis))?;
    }

    #[test]
    fn plus_millis_underflow((duration, millis) in arb_duration_underflow_units(NANOSECONDS_IN_MILLISECOND)) {
        expect_panic("addition of millis overflowed", || duration.plus_millis(millis))?;
    }

    #[test]
    fn plus_millis_specific((base, amount, expected_seconds, expected_nano)
        in prop_oneof!(
            Just((Duration::of_seconds_and_adjustment(0, 0), 0, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1, 0, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 999, 0, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1000, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1001, 1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1999, 1, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 2000, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1, -1, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -999, -1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1000, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1001, -2, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1999, -2, 1000000)),

            Just((Duration::of_seconds_and_adjustment(0, 1), 0, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1, 0, 1000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 998, 0, 998000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 999, 0, 999000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1000, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1998, 1, 998000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1999, 1, 999000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 2000, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -1, -1, 999000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -2, -1, 998000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -1000, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -1001, -2, 999000001)),

            Just((Duration::of_seconds_and_adjustment(0, 1000000), 0, 0, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1, 0, 2000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 998, 0, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 999, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1000, 1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1998, 1, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1999, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 2000, 2, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -2, -1, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -999, -1, 2000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1000, -1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1001, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1002, -2, 999000000)),

            Just((Duration::of_seconds_and_adjustment(0, 999999999), 0, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 1, 1, 999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 999, 1, 998999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 1000, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 1001, 2, 999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), -1, 0, 998999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), -1000, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), -1001, -1, 998999999)),
        )) {
        let added = base.plus_millis(amount);
        prop_assert_eq!(added.seconds(), expected_seconds);
        prop_assert_eq!(added.nano(), expected_nano);
    }

    #[test]
    fn plus_zero_nanos(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_nanos(0));
    }

    #[test]
    fn identity_duration_plus_nanos(nanos in i64::MIN..i64::MAX) {
        let added = next(Duration::ZERO.plus_nanos(nanos));
        let other = next(Duration::ZERO).plus_nanos(nanos);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_nanos((duration, nanos) in arb_duration_remaining_units(1)) {
        let added = next(duration.plus_nanos(nanos));
        let other = next(duration).plus_nanos(nanos);
        prop_assert_eq!(added, other);
    }

    #[test]
    fn plus_nanos_overflow((duration, nanos) in arb_duration_overflow_units(1)) {
        expect_panic("addition of nanos overflowed", || duration.plus_nanos(nanos))?;
    }

    #[test]
    fn plus_nanos_underflow((duration, nanos) in arb_duration_underflow_units(1)) {
        expect_panic("addition of nanos overflowed", || duration.plus_nanos(nanos))?;
    }

    #[test]
    fn plus_nanos_specific((base, amount, expected_seconds, expected_nano)
        in prop_oneof!(
            Just((Duration::of_seconds_and_adjustment(0, 0), 0, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 999999999, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1000000000, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1000000001, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1999999999, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 2000000000, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -999999999, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1000000000, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1000000001, -2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1999999999, -2, 1)),

            Just((Duration::of_seconds_and_adjustment(1, 0), 0, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 999999999, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1000000000, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1000000001, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1999999999, 2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 2000000000, 3, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -999999999, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1000000000, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1000000001, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1999999999, -1, 1)),

            Just((Duration::of_seconds_and_adjustment(-1, 0), 0, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 999999999, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1000000000, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1000000001, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1999999999, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 2000000000, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1, -2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -999999999, -2, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1000000000, -2, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1000000001, -3, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1999999999, -3, 1)),

            Just((Duration::of_seconds_and_adjustment(1, 1), 0, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1, 1, 2)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 999999998, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 999999999, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1000000000, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1999999998, 2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1999999999, 3, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 2000000000, 3, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -2, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1000000000, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1000000001, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1000000002, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -2000000000, -1, 1)),

            Just((Duration::of_seconds_and_adjustment(1, 999999999), 0, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 1, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 999999999, 2, 999999998)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 1000000000, 2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 1000000001, 3, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1, 1, 999999998)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1000000000, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1000000001, 0, 999999998)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1999999999, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -2000000000, -1, 999999999)),

            Just((Duration::of_seconds_and_adjustment(i64::MAX, 0), 999999999, i64::MAX, 999999999)),
            Just((Duration::of_seconds_and_adjustment(i64::MAX - 1, 0), 1999999999, i64::MAX, 999999999)),
            Just((Duration::of_seconds_and_adjustment(i64::MIN, 1), -1, i64::MIN, 0)),
            Just((Duration::of_seconds_and_adjustment(i64::MIN + 1, 1), -1000000001, i64::MIN, 0)),
        )) {
        let added = base.plus_nanos(amount);
        prop_assert_eq!(added.seconds(), expected_seconds);
        prop_assert_eq!(added.nano(), expected_nano);
    }
}
