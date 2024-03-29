use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

use crate::assert::expect_panic;

proptest! {
    #[test]
    fn of_days(days in (Duration::MIN.seconds() / SECONDS_IN_DAY)-1..(Duration::MAX.seconds() / SECONDS_IN_DAY)) {
        let duration = Duration::of_days(days + 1);
        prop_assert_eq!(0, duration.nano());
        prop_assert_eq!((days + 1) * SECONDS_IN_DAY, duration.seconds());
    }
}

proptest! {
    #[test]
    fn of_days_overflow(days in Duration::MAX.seconds() / SECONDS_IN_DAY + 1..=i64::MAX) {
        expect_panic("days would overflow duration", || Duration::of_days(days))?;
    }
}

proptest! {
    #[test]
    fn of_days_underflow(days in i64::MIN..Duration::MIN.seconds() / SECONDS_IN_DAY) {
        expect_panic("days would overflow duration", || Duration::of_days(days))?;
    }
}

proptest! {
    #[test]
    fn of_hours(hours in (i64::min_value() / SECONDS_IN_HOUR)-1..(i64::max_value() / SECONDS_IN_HOUR)) {
        let duration = Duration::of_hours(hours + 1);
        prop_assert_eq!(0, duration.nano());
        prop_assert_eq!((hours + 1) * SECONDS_IN_HOUR, duration.seconds());
    }
}

proptest! {
    #[test]
    fn of_hours_overflow(hours in Duration::MAX.seconds() / SECONDS_IN_HOUR + 1..=i64::MAX) {
        expect_panic("hours would overflow duration", || Duration::of_hours(hours))?;
    }
}

proptest! {
    #[test]
    fn of_hours_underflow(hours in i64::MIN..Duration::MIN.seconds() / SECONDS_IN_HOUR) {
        expect_panic("hours would overflow duration", || Duration::of_hours(hours))?;
    }
}

proptest! {
    #[test]
    fn of_minutes(minutes in (i64::min_value() / SECONDS_IN_MINUTE)-1..(i64::max_value() / SECONDS_IN_MINUTE)) {
        let duration = Duration::of_minutes(minutes + 1);
        prop_assert_eq!(0, duration.nano());
        prop_assert_eq!((minutes + 1) * SECONDS_IN_MINUTE, duration.seconds());
    }
}

proptest! {
    #[test]
    fn of_minutes_overflow(minutes in Duration::MAX.seconds() / SECONDS_IN_MINUTE + 1..=i64::MAX) {
        expect_panic("minutes would overflow duration", || Duration::of_minutes(minutes))?;
    }
}

proptest! {
    #[test]
    fn of_minutes_underflow(minutes in i64::MIN..Duration::MIN.seconds() / SECONDS_IN_MINUTE) {
        expect_panic("minutes would overflow duration", || Duration::of_minutes(minutes))?;
    }
}

const DURATION_SAFE_UPPER_BOUND: i64 = Duration::MAX.seconds() - i64::MAX / NANOSECONDS_IN_SECOND;
const DURATION_SAFE_LOWER_BOUND: i64 =
    Duration::MIN.seconds() - i64::MIN / NANOSECONDS_IN_SECOND + 1;

prop_compose! {
    fn seconds_and_bounds()
        (seconds in prop::num::i64::ANY) -> (i64, i64, i64)
        {
            let upper = if seconds >= DURATION_SAFE_UPPER_BOUND {
                (Duration::MAX.seconds() - seconds + 1) * NANOSECONDS_IN_SECOND - 1
            } else {
                i64::MAX
            };

            let lower = if seconds < DURATION_SAFE_LOWER_BOUND {
                (Duration::MIN.seconds() - seconds) * NANOSECONDS_IN_SECOND
            } else {
                i64::MIN
            };

            (seconds, lower, upper)
        }
}

prop_compose! {
    fn seconds_and_adjustment()
        ((seconds, lower, upper) in seconds_and_bounds())
            (seconds in Just(seconds), adjustment in lower..=upper) -> (i64, i64)
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

        prop_assert_eq!(expected_nanos as u32, duration.nano());
        prop_assert_eq!(expected_seconds, duration.seconds());
    }
}

prop_compose! {
    fn adjustment_overflow()
        (seconds in DURATION_SAFE_UPPER_BOUND + 1..=Duration::MAX.seconds())
            (seconds in Just(seconds), adjustment in (Duration::MAX.seconds() - seconds + 1) * NANOSECONDS_IN_SECOND..=i64::MAX)-> (i64, i64)
            {
                (seconds, adjustment)
            }
}

proptest! {
    #[test]
    fn of_seconds_and_adjustment_overflow((seconds, nanoseconds) in adjustment_overflow()) {
        expect_panic("nano adjustment would overflow duration", || Duration::of_seconds_and_adjustment(seconds, nanoseconds))?;
    }
}

prop_compose! {
    fn adjustment_underflow()
        (seconds in Duration::MIN.seconds()..DURATION_SAFE_LOWER_BOUND)
            (seconds in Just(seconds), adjustment in i64::MIN..=(i64::MIN - seconds - 1) * NANOSECONDS_IN_SECOND)-> (i64, i64)
            {
                (seconds, adjustment)
            }
}

proptest! {
    #[test]
    fn of_seconds_and_adjustment_underflow((seconds, nanoseconds) in adjustment_underflow()) {
        expect_panic("nano adjustment would overflow duration", || Duration::of_seconds_and_adjustment(seconds, nanoseconds))?;
    }
}

proptest! {
    #[test]
    fn of_seconds(seconds in prop::num::i64::ANY) {
        let duration = Duration::of_seconds(seconds);

        prop_assert_eq!(0, duration.nano());
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

        prop_assert_eq!(nanos as u32, duration.nano());
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

        prop_assert_eq!(expected_nanos as u32, duration.nano());
        prop_assert_eq!(seconds, duration.seconds());
    }
}
