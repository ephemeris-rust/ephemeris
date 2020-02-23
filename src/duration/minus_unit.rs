use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

use crate::assert::expect_panic;

use crate::duration::test_util::*;

proptest! {
    #[test]
    fn minus_zero_days(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.minus_days(0));
    }

    #[test]
    fn identity_duration_minus_days(days in i64::MIN / SECONDS_IN_DAY..i64::MAX / SECONDS_IN_DAY) {
        let subtracted = next(Duration::ZERO.minus_days(days));
        let other = next(Duration::ZERO).minus_days(days);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_days((duration, days) in arb_duration_remaining_units(NANOSECONDS_IN_DAY).prop_map(|(dur, day)| (dur, -day))) {
        let subtracted = next(duration.minus_days(days));
        let other = next(duration).minus_days(days);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_days_overflow((duration, days) in arb_duration_overflow_units(NANOSECONDS_IN_DAY)) {
        expect_panic("subtraction of days overflowed", || duration.minus_days(-days))?;
    }

    #[test]
    fn minus_days_underflow((duration, days) in arb_duration_underflow_units(NANOSECONDS_IN_DAY)) {
        expect_panic("subtraction of days overflowed", || duration.minus_days(-days))?;
    }

    #[test]
    fn minus_days_specific((base, amount, expected)
        in prop_oneof!(
            Just((Duration::of_days(0), 0, 0)),
            Just((Duration::of_days(0), 1, -1)),
            Just((Duration::of_days(0), -1, 1)),
            Just((Duration::of_days(i64::MAX / SECONDS_IN_DAY), 0, i64::MAX / SECONDS_IN_DAY)),
            Just((Duration::of_days(i64::MIN / SECONDS_IN_DAY), 0, i64::MIN / SECONDS_IN_DAY)),
            Just((Duration::of_days(1), 0, 1)),
            Just((Duration::of_days(1), 1, 0)),
            Just((Duration::of_days(1), -1, 2)),
            Just((Duration::of_days(i64::MAX / SECONDS_IN_DAY), 1, i64::MAX / SECONDS_IN_DAY - 1)),
            Just((Duration::of_days(i64::MIN / SECONDS_IN_DAY), -1, i64::MIN / SECONDS_IN_DAY + 1)),
            Just((Duration::of_days(-1), 0, -1)),
            Just((Duration::of_days(-1), 1, -2)),
            Just((Duration::of_days(-1), -1, 0)),
        )) {
        let subtracted = base.minus_days(amount);
        prop_assert_eq!(subtracted.to_days(), expected);
    }

    #[test]
    fn minus_zero_hours(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.minus_hours(0));
    }

    #[test]
    fn identity_duration_minus_hours(hours in i64::MIN / SECONDS_IN_HOUR..i64::MAX / SECONDS_IN_HOUR) {
        let subtracted = next(Duration::ZERO.minus_hours(hours));
        let other = next(Duration::ZERO).minus_hours(hours);
        prop_assert_eq!(subtracted, other);
    }
}

proptest! {
    #[test]
    fn minus_hours((duration, hours) in arb_duration_remaining_units(NANOSECONDS_IN_HOUR).prop_map(|(dur, hour)| (dur, -hour))) {
        let subtracted = next(duration.minus_hours(hours));
        let other = next(duration).minus_hours(hours);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_hours_overflow((duration, hours) in arb_duration_overflow_units(NANOSECONDS_IN_HOUR)) {
        expect_panic("subtraction of hours overflowed", || duration.minus_hours(-hours))?;
    }

    #[test]
    fn minus_hours_underflow((duration, hours) in arb_duration_underflow_units(NANOSECONDS_IN_HOUR)) {
        expect_panic("subtraction of hours overflowed", || duration.minus_hours(-hours))?;
    }

    #[test]
    fn minus_hours_specific((base, amount, expected)
        in prop_oneof!(
            Just((Duration::of_hours(0), 0, 0)),
            Just((Duration::of_hours(0), 1, -1)),
            Just((Duration::of_hours(0), -1, 1)),
            Just((Duration::of_hours(i64::MAX / SECONDS_IN_HOUR), 0, i64::MAX / SECONDS_IN_HOUR)),
            Just((Duration::of_hours(i64::MIN / SECONDS_IN_HOUR), 0, i64::MIN / SECONDS_IN_HOUR)),
            Just((Duration::of_hours(1), 0, 1)),
            Just((Duration::of_hours(1), 1, 0)),
            Just((Duration::of_hours(1), -1, 2)),
            Just((Duration::of_hours(i64::MAX / SECONDS_IN_HOUR), 1, i64::MAX / SECONDS_IN_HOUR - 1)),
            Just((Duration::of_hours(i64::MIN / SECONDS_IN_HOUR), -1, i64::MIN / SECONDS_IN_HOUR + 1)),
            Just((Duration::of_hours(-1), 0, -1)),
            Just((Duration::of_hours(-1), 1, -2)),
            Just((Duration::of_hours(-1), -1, 0)),
        )) {
        let subtracted = base.minus_hours(amount);
        prop_assert_eq!(subtracted.to_hours(), expected);
    }

    #[test]
    fn minus_zero_minutes(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.minus_minutes(0));
    }

    #[test]
    fn identity_duration_minus_minutes(minutes in i64::MIN / SECONDS_IN_MINUTE..i64::MAX / SECONDS_IN_MINUTE) {
        let subtracted = next(Duration::ZERO.minus_minutes(minutes));
        let other = next(Duration::ZERO).minus_minutes(minutes);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_minutes((duration, minutes) in arb_duration_remaining_units(NANOSECONDS_IN_MINUTE).prop_map(|(dur, min)| (dur, -min))) {
        let subtracted = next(duration.minus_minutes(minutes));
        let other = next(duration).minus_minutes(minutes);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_minutes_overflow((duration, minutes) in arb_duration_overflow_units(NANOSECONDS_IN_MINUTE)) {
        expect_panic("subtraction of minutes overflowed", || duration.minus_minutes(-minutes))?;
    }

    #[test]
    fn minus_minutes_underflow((duration, minutes) in arb_duration_underflow_units(NANOSECONDS_IN_MINUTE)) {
        expect_panic("subtraction of minutes overflowed", || duration.minus_minutes(-minutes))?;
    }

    #[test]
    fn minus_minutes_specific((base, amount, expected)
        in prop_oneof!(
            Just((Duration::of_minutes(0), 0, 0)),
            Just((Duration::of_minutes(0), 1, -1)),
            Just((Duration::of_minutes(0), -1, 1)),
            Just((Duration::of_minutes(i64::MAX / SECONDS_IN_MINUTE), 0, i64::MAX / SECONDS_IN_MINUTE)),
            Just((Duration::of_minutes(i64::MIN / SECONDS_IN_MINUTE), 0, i64::MIN / SECONDS_IN_MINUTE)),
            Just((Duration::of_minutes(1), 0, 1)),
            Just((Duration::of_minutes(1), 1, 0)),
            Just((Duration::of_minutes(1), -1, 2)),
            Just((Duration::of_minutes(i64::MAX / SECONDS_IN_MINUTE), 1, i64::MAX / SECONDS_IN_MINUTE - 1)),
            Just((Duration::of_minutes(i64::MIN / SECONDS_IN_MINUTE), -1, i64::MIN / SECONDS_IN_MINUTE + 1)),
            Just((Duration::of_minutes(-1), 0, -1)),
            Just((Duration::of_minutes(-1), 1, -2)),
            Just((Duration::of_minutes(-1), -1, 0)),
        )) {
        let subtracted = base.minus_minutes(amount);
        prop_assert_eq!(subtracted.to_minutes(), expected);
    }

    #[test]
    fn minus_zero_seconds(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.minus_seconds(0));
    }

    #[test]
    fn identity_duration_minus_seconds(seconds in i64::MIN..i64::MAX) {
        let subtracted = next(Duration::ZERO.minus_seconds(seconds));
        let other = next(Duration::ZERO).minus_seconds(seconds);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_seconds((duration, seconds) in arb_duration_remaining_units(NANOSECONDS_IN_SECOND).prop_map(|(dur, sec)| (dur, -sec))) {
        let subtracted = next(duration.minus_seconds(seconds));
        let other = next(duration).minus_seconds(seconds);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_seconds_overflow((duration, seconds) in arb_duration_overflow_units(NANOSECONDS_IN_SECOND)) {
        expect_panic("subtraction of seconds overflowed", || duration.minus_seconds(-seconds))?;
    }

    #[test]
    fn minus_seconds_underflow((duration, seconds) in arb_duration_underflow_units(NANOSECONDS_IN_SECOND)) {
        expect_panic("subtraction of seconds overflowed", || duration.minus_seconds(-seconds))?;
    }

    #[test]
    fn minus_seconds_specific((base, amount, expected_seconds, expected_nano)
        in prop_oneof!(
            Just((Duration::of_seconds(0), 0, 0, 0)),
            Just((Duration::of_seconds(0), 1, -1, 0)),
            Just((Duration::of_seconds(0), -1, 1, 0)),
            Just((Duration::of_seconds(0), i64::MAX, -i64::MAX, 0)),
            Just((Duration::of_seconds(0), i64::MIN + 1, i64::MAX, 0)),
            Just((Duration::of_seconds(1), 0, 1, 0)),
            Just((Duration::of_seconds(1), 1, 0, 0)),
            Just((Duration::of_seconds(1), -1, 2, 0)),
            Just((Duration::of_seconds(1), i64::MAX - 1, -i64::MAX + 2, 0)),
            Just((Duration::of_seconds(1), i64::MIN + 2, i64::MAX, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 0, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), i64::MAX, -i64::MAX + 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), i64::MIN + 2, i64::MAX, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), 0, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), 1, -2, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), -1, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), i64::MAX, i64::MIN, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 1), i64::MIN + 1, i64::MAX - 1, 1)),
        )) {
        let subtracted = base.minus_seconds(amount);
        prop_assert_eq!(subtracted.seconds(), expected_seconds);
        prop_assert_eq!(subtracted.nano(), expected_nano);
    }

    #[test]
    fn minus_zero_millis(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.minus_millis(0));
    }

    #[test]
    fn identity_duration_minus_millis(millis in i64::MIN..i64::MAX) {
        let subtracted = next(Duration::ZERO.minus_millis(millis));
        let other = next(Duration::ZERO).minus_millis(millis);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_millis((duration, millis) in arb_duration_remaining_units(NANOSECONDS_IN_MILLISECOND).prop_map(|(dur, milli)| (dur, -milli))) {
        let subtracted = next(duration.minus_millis(millis));
        let other = next(duration).minus_millis(millis);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_millis_overflow((duration, millis) in arb_duration_overflow_units(NANOSECONDS_IN_MILLISECOND)) {
        expect_panic("subtraction of millis overflowed", || duration.minus_millis(-millis))?;
    }

    #[test]
    fn minus_millis_underflow((duration, millis) in arb_duration_underflow_units(NANOSECONDS_IN_MILLISECOND)) {
        expect_panic("subtraction of millis overflowed", || duration.minus_millis(-millis))?;
    }

    #[test]
    fn minus_millis_specific((base, amount, expected_seconds, expected_nano)
        in prop_oneof!(
            Just((Duration::of_seconds_and_adjustment(0, 0), 0, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1, -1, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 999, -1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1000, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1001, -2, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1999, -2, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 2000, -2, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1, 0, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -999, 0, 999000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1000, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1001, 1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1999, 1, 999000000)),

            Just((Duration::of_seconds_and_adjustment(0, 1), 0, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1, -1, 999000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 998, -1, 2000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 999, -1, 1000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1000, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1998, -2, 2000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 1999, -2, 1000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), 2000, -2, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -1, 0, 1000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -2, 0, 2000001)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -1000, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 1), -1001, 1, 1000001)),

            Just((Duration::of_seconds_and_adjustment(0, 1000000), 0, 0, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 998, -1, 3000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 999, -1, 2000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1000, -1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1998, -2, 3000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 1999, -2, 2000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), 2000, -2, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1, 0, 2000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -2, 0, 3000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -999, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1000, 1, 1000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1001, 1, 2000000)),
            Just((Duration::of_seconds_and_adjustment(0, 1000000), -1002, 1, 3000000)),

            Just((Duration::of_seconds_and_adjustment(0, 999999999), 0, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 1, 0, 998999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 999, 0, 999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 1000, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), 1001, -1, 998999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), -1, 1, 999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), -1000, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 999999999), -1001, 2, 999999)),
        )) {
        let subtracted = base.minus_millis(amount);
        prop_assert_eq!(subtracted.seconds(), expected_seconds);
        prop_assert_eq!(subtracted.nano(), expected_nano);
    }

    #[test]
    fn minus_zero_nanos(duration in arb_duration()) {
        prop_assert_eq!(duration, duration.minus_nanos(0));
    }

    #[test]
    fn identity_duration_minus_nanos(nanos in i64::MIN..i64::MAX) {
        let subtracted = next(Duration::ZERO.minus_nanos(nanos));
        let other = next(Duration::ZERO).minus_nanos(nanos);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_nanos((duration, nanos) in arb_duration_remaining_units(1).prop_map(|(dur, nano)| (dur, -nano))) {
        let subtracted = next(duration.minus_nanos(nanos));
        let other = next(duration).minus_nanos(nanos);
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn minus_nanos_overflow((duration, nanos) in arb_duration_overflow_units(1)) {
        expect_panic("subtraction of nanos overflowed", || duration.minus_nanos(-nanos))?;
    }

    #[test]
    fn minus_nanos_underflow((duration, nanos) in arb_duration_underflow_units(1)) {
        expect_panic("subtraction of nanos overflowed", || duration.minus_nanos(-nanos))?;
    }

    #[test]
    fn minus_nanos_specific((base, amount, expected_seconds, expected_nano)
        in prop_oneof!(
            Just((Duration::of_seconds_and_adjustment(0, 0), 0, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 999999999, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1000000000, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1000000001, -2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 1999999999, -2, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), 2000000000, -2, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -999999999, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1000000000, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1000000001, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(0, 0), -1999999999, 1, 999999999)),

            Just((Duration::of_seconds_and_adjustment(1, 0), 0, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 999999999, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1000000000, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1000000001, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 1999999999, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), 2000000000, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -999999999, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1000000000, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1000000001, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 0), -1999999999, 2, 999999999)),

            Just((Duration::of_seconds_and_adjustment(-1, 0), 0, -1, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1, -2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 999999999, -2, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1000000000, -2, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1000000001, -3, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 1999999999, -3, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), 2000000000, -3, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -999999999, -1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1000000000, 0, 0)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1000000001, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(-1, 0), -1999999999, 0, 999999999)),

            Just((Duration::of_seconds_and_adjustment(1, 1), 0, 1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 999999998, 0, 3)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 999999999, 0, 2)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1000000000, 0, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1999999998, -1, 3)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 1999999999, -1, 2)),
            Just((Duration::of_seconds_and_adjustment(1, 1), 2000000000, -1, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1, 1, 2)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -2, 1, 3)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1000000000, 2, 1)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1000000001, 2, 2)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -1000000002, 2, 3)),
            Just((Duration::of_seconds_and_adjustment(1, 1), -2000000000, 3, 1)),

            Just((Duration::of_seconds_and_adjustment(1, 999999999), 0, 1, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 1, 1, 999999998)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 999999999, 1, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 1000000000, 0, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), 1000000001, 0, 999999998)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1, 2, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1000000000, 2, 999999999)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1000000001, 3, 0)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -1999999999, 3, 999999998)),
            Just((Duration::of_seconds_and_adjustment(1, 999999999), -2000000000, 3, 999999999)),

            Just((Duration::of_seconds_and_adjustment(i64::MAX, 0), -999999999, i64::MAX, 999999999)),
            Just((Duration::of_seconds_and_adjustment(i64::MAX - 1, 0), -1999999999, i64::MAX, 999999999)),
            Just((Duration::of_seconds_and_adjustment(i64::MIN, 1), 1, i64::MIN, 0)),
            Just((Duration::of_seconds_and_adjustment(i64::MIN + 1, 1), 1000000001, i64::MIN, 0)),
        )) {
        let subtracted = base.minus_nanos(amount);
        prop_assert_eq!(subtracted.seconds(), expected_seconds);
        prop_assert_eq!(subtracted.nano(), expected_nano);
    }
}
