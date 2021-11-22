use proptest::prelude::*;

use crate::{assert::expect_panic, constants::*, test_util::*, Duration};

proptest! {
    #[test]
    fn to_days((days, duration) in units_in_type_range(SECONDS_IN_DAY).prop_map(|days| (days, Duration::of_days(days)))) {
        prop_assert_eq!(days, duration.to_days());
    }

    #[test]
    fn to_hours((hours, duration) in units_in_type_range(SECONDS_IN_HOUR).prop_map(|hours| (hours, Duration::of_hours(hours)))) {
        prop_assert_eq!(hours, duration.to_hours());
    }

    #[test]
    fn to_minutes((minutes, duration) in units_in_type_range(SECONDS_IN_MINUTE).prop_map(|minutes| (minutes, Duration::of_minutes(minutes)))) {
        prop_assert_eq!(minutes, duration.to_minutes());
    }
}

fn get_nanos_range(seconds: i64, units_in_second: i64, nanos_in_unit: i64) -> std::ops::Range<i64> {
    let upper = i64::MAX / units_in_second;
    if seconds == upper {
        (i64::MAX % units_in_second + 1) * nanos_in_unit..units_in_second * nanos_in_unit
    } else if seconds == i64::MIN / units_in_second - 1 {
        0..(i64::MIN % units_in_second) * nanos_in_unit
    } else {
        0..units_in_second * nanos_in_unit
    }
}

prop_compose! {
    fn millis_overflow()
        (seconds in Duration::MAX.seconds() / MILLISECONDS_IN_SECOND..=Duration::MAX.seconds())
            (seconds in Just(seconds), nano in get_nanos_range(seconds, MILLISECONDS_IN_SECOND, NANOSECONDS_IN_MILLISECOND)) -> Duration
            {
                Duration::of_seconds_and_adjustment(seconds, nano)
            }
}

prop_compose! {
    fn millis_underflow()
    (seconds in Duration::MIN.seconds()..Duration::MIN.seconds() / MILLISECONDS_IN_SECOND)
        (seconds in Just(seconds), nano in get_nanos_range(seconds, MILLISECONDS_IN_SECOND, NANOSECONDS_IN_MILLISECOND))-> Duration
        {
            Duration::of_seconds_and_adjustment(seconds, nano)
        }
}

proptest! {
    #[test]
    fn to_millis((millis, duration) in any::<i64>().prop_map(|millis| (millis, Duration::of_millis(millis)))) {
        prop_assert_eq!(millis, duration.to_millis());
    }

    #[test]
    fn to_millis_overflow(duration in millis_overflow()) {
        expect_panic("total milliseconds would overflow", || duration.to_millis())?;
    }

    #[test]
    fn to_millis_underflow(duration in millis_underflow()) {
        expect_panic("total milliseconds would overflow", || duration.to_millis())?;
    }
}

prop_compose! {
    fn nanos_overflow()
        (seconds in Duration::MAX.seconds() / NANOSECONDS_IN_SECOND..=Duration::MAX.seconds())
            (seconds in Just(seconds), nano in get_nanos_range(seconds, NANOSECONDS_IN_SECOND, 1)) -> Duration
            {
                Duration::of_seconds_and_adjustment(seconds, nano)
            }
}

prop_compose! {
    fn nanos_underflow()
    (seconds in Duration::MIN.seconds()..Duration::MIN.seconds() / NANOSECONDS_IN_SECOND)
        (seconds in Just(seconds), nano in get_nanos_range(seconds, NANOSECONDS_IN_SECOND, 1))-> Duration
        {
            Duration::of_seconds_and_adjustment(seconds, nano)
        }
}

proptest! {
    #[test]
    fn to_nanos((nanos, duration) in any::<i64>().prop_map(|nanos| (nanos, Duration::of_nanos(nanos)))) {
        prop_assert_eq!(nanos, duration.to_nanos());
    }

    #[test]
    fn to_nanos_overflow(duration in nanos_overflow()) {
        expect_panic("total nanoseconds would overflow", || duration.to_nanos())?;
    }

    #[test]
    fn to_nanos_underflow(duration in nanos_underflow()) {
        expect_panic("total nanoseconds would overflow", || duration.to_nanos())?;
    }
}
