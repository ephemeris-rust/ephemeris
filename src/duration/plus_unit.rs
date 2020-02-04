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
    fn identity_duration_plus_days(days in i64::min_value() / SECONDS_IN_DAY..i64::max_value() / SECONDS_IN_DAY) {
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
    fn plus_zero_hours(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_hours(0));
    }

    #[test]
    fn identity_duration_plus_hours(hours in i64::min_value() / SECONDS_IN_HOUR..i64::max_value() / SECONDS_IN_HOUR) {
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
    fn plus_zero_minutes(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_minutes(0));
    }

    #[test]
    fn identity_duration_plus_minutes(minutes in i64::min_value() / SECONDS_IN_MINUTE..i64::max_value() / SECONDS_IN_MINUTE) {
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
    fn plus_zero_seconds(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_seconds(0));
    }

    #[test]
    fn identity_duration_plus_seconds(seconds in i64::min_value()..i64::max_value()) {
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
    fn plus_zero_nanos(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.plus_nanos(0));
    }

    #[test]
    fn identity_duration_plus_nanos(nanos in i64::min_value()..i64::max_value()) {
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
}
