use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

proptest! {
    #[test]
    fn of_days(days in (i64::min_value() / SECONDS_IN_DAY)-1..(i64::max_value() / SECONDS_IN_DAY)) {
        let duration = Duration::of_days(days + 1);
        prop_assert_eq!(0, duration.nanos());
        prop_assert_eq!((days + 1) * SECONDS_IN_DAY, duration.seconds());
    }
}

// TODO: Generate input via proptest
#[test]
#[should_panic(expected = "days would overflow duration")]
fn of_days_overflow() {
    Duration::of_days(i64::max_value() / SECONDS_IN_DAY + 1);
}

#[test]
#[should_panic(expected = "days would overflow duration")]
fn of_days_underflow() {
    Duration::of_days(i64::min_value() / SECONDS_IN_DAY - 1);
}

proptest! {
    #[test]
    fn of_hours(hours in (i64::min_value() / SECONDS_IN_HOUR)-1..(i64::max_value() / SECONDS_IN_HOUR)) {
        let duration = Duration::of_hours(hours + 1);
        prop_assert_eq!(0, duration.nanos());
        prop_assert_eq!((hours + 1) * SECONDS_IN_HOUR, duration.seconds());
    }
}

// TODO: Generate input via proptest
#[test]
#[should_panic(expected = "hours would overflow duration")]
fn of_hours_overflow() {
    Duration::of_hours(i64::max_value() / SECONDS_IN_HOUR + 1);
}

#[test]
#[should_panic(expected = "hours would overflow duration")]
fn of_hours_underflow() {
    Duration::of_hours(i64::min_value() / SECONDS_IN_HOUR - 1);
}

proptest! {
    #[test]
    fn of_minutes(minutes in (i64::min_value() / SECONDS_IN_MINUTE)-1..(i64::max_value() / SECONDS_IN_MINUTE)) {
        let duration = Duration::of_minutes(minutes + 1);
        prop_assert_eq!(0, duration.nanos());
        prop_assert_eq!((minutes + 1) * SECONDS_IN_MINUTE, duration.seconds());
    }
}

// TODO: Generate input via proptest
#[test]
#[should_panic(expected = "minutes would overflow duration")]
fn of_minutes_overflow() {
    Duration::of_minutes(i64::max_value() / SECONDS_IN_MINUTE + 1);
}

#[test]
#[should_panic(expected = "minutes would overflow duration")]
fn of_minutes_underflow() {
    Duration::of_minutes(i64::min_value() / SECONDS_IN_MINUTE - 1);
}

prop_compose! {
    fn seconds_and_bounds()
        (seconds in prop::num::i64::ANY) -> (i64, i64, i64)
        {
            const UPPER_BOUND: i64 = i64::max_value() - i64::max_value() / NANOSECONDS_IN_SECOND;
            const LOWER_BOUND: i64 = i64::min_value() - i64::min_value() / NANOSECONDS_IN_SECOND + 1;

            let upper = match seconds {
                UPPER_BOUND...i64::MAX => (i64::max_value() - seconds + 1) * NANOSECONDS_IN_SECOND - 1,
                _ => i64::max_value(),
            };

            let lower = match seconds {
                i64::MIN...LOWER_BOUND => (i64:: min_value() - seconds) * NANOSECONDS_IN_SECOND,
                _ => i64::min_value(),
            };

            (seconds, lower, upper)
        }
}

prop_compose! {
    fn seconds_and_adjustment()
        ((seconds, lower, upper) in seconds_and_bounds())
            // TODO: Inclusive upper bound
            (seconds in Just(seconds), adjustment in lower..upper) -> (i64, i64)
            {
                (seconds, adjustment)
            }
}

proptest! {
    #[test]
    fn of_seconds_and_adjustment((seconds, adjustment) in seconds_and_adjustment()) {
        let duration = Duration::of_seconds_and_adjustment(seconds, adjustment);

        let (expected_seconds, expected_nanos) = if adjustment >= 0 {
            (seconds + adjustment / NANOSECONDS_IN_SECOND, adjustment % NANOSECONDS_IN_SECOND)
        } else {
            (seconds + adjustment / NANOSECONDS_IN_SECOND - 1, adjustment % NANOSECONDS_IN_SECOND + NANOSECONDS_IN_SECOND)
        };

        prop_assert_eq!(expected_nanos as u32, duration.nanos());
        prop_assert_eq!(expected_seconds, duration.seconds());
    }
}

// TODO: Generate input via proptest
#[test]
#[should_panic(expected = "nano adjustment would overflow duration")]
fn of_seconds_and_adjustment_overflow() {
    let seconds = i64::max_value() - i64::max_value() / NANOSECONDS_IN_SECOND + 1;
    Duration::of_seconds_and_adjustment(
        seconds,
        (i64::max_value() - seconds + 1) * NANOSECONDS_IN_SECOND,
    );
}

#[test]
#[should_panic(expected = "nano adjustment would overflow duration")]
fn of_seconds_and_adjustment_underflow() {
    let seconds = i64::min_value() - i64::min_value() / NANOSECONDS_IN_SECOND - 1;
    Duration::of_seconds_and_adjustment(
        seconds,
        (i64::min_value() - seconds - 1) * NANOSECONDS_IN_SECOND,
    );
}

proptest! {
    #[test]
    fn of_seconds(seconds in prop::num::i64::ANY) {
        let duration = Duration::of_seconds(seconds);

        prop_assert_eq!(0, duration.nanos());
        prop_assert_eq!(seconds, duration.seconds());
    }
}

proptest! {
    #[test]
    fn of_millis(millis in prop::num::i64::ANY) {
        let duration = Duration::of_millis(millis);

        let (seconds, nanos) = if millis >= 0 {
            (millis / MILLISECONDS_IN_SECOND, millis % MILLISECONDS_IN_SECOND * NANOSECONDS_IN_MILLISECOND)
        } else if millis % MILLISECONDS_IN_SECOND == 0 {
            (millis / MILLISECONDS_IN_SECOND, 0)
        } else {
            (millis / MILLISECONDS_IN_SECOND - 1, millis % MILLISECONDS_IN_SECOND * NANOSECONDS_IN_MILLISECOND + NANOSECONDS_IN_SECOND)
        };

        prop_assert_eq!(nanos as u32, duration.nanos());
        prop_assert_eq!(seconds, duration.seconds());
    }
}

proptest! {
    #[test]
    fn of_nanos(nanos in prop::num::i64::ANY) {
        let duration = Duration::of_nanos(nanos);

        let (seconds, expected_nanos) = if nanos >= 0 {
            (nanos / NANOSECONDS_IN_SECOND, nanos % NANOSECONDS_IN_SECOND)
        } else if nanos % NANOSECONDS_IN_SECOND == 0 {
            (nanos / NANOSECONDS_IN_SECOND, 0)
        } else {
            (nanos / NANOSECONDS_IN_SECOND - 1, nanos % NANOSECONDS_IN_SECOND + NANOSECONDS_IN_SECOND)
        };

        prop_assert_eq!(expected_nanos as u32, duration.nanos());
        prop_assert_eq!(seconds, duration.seconds());
    }
}
