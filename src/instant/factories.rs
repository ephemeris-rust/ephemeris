use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Instant;

proptest! {
    #[test]
    fn of_second(seconds in prop::num::i64::ANY) {
        let instant = Instant::of_epoch_second(seconds);

        prop_assert_eq!(0, instant.nano());
        prop_assert_eq!(seconds, instant.epoch_second());
    }
}

prop_compose! {
    fn seconds_and_bounds()
        (seconds in prop::num::i64::ANY) -> (i64, i64, i64)
        {
            let upper_bound = Instant::MAX.epoch_second() - i64::MAX / NANOSECONDS_IN_SECOND;
            let lower_bound = Instant::MIN.epoch_second() - i64::MIN / NANOSECONDS_IN_SECOND + 1;

            let upper = if seconds >= upper_bound {
                (Instant::MAX.epoch_second() - seconds + 1) * NANOSECONDS_IN_SECOND - 1
            } else {
                i64::MAX
            };

            let lower = if seconds < lower_bound {
                (Instant::MIN.epoch_second() - seconds) * NANOSECONDS_IN_SECOND
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
    fn of_second_and_adjustment_adjusted((seconds, adjustment) in seconds_and_adjustment()) {
        let instant = Instant::of_epoch_second_and_adjustment(seconds, adjustment);

        let (expected_seconds, expected_nanos) = if adjustment >= 0 {
            (seconds + adjustment / NANOSECONDS_IN_SECOND, adjustment % NANOSECONDS_IN_SECOND)
        } else {
            (seconds + adjustment / NANOSECONDS_IN_SECOND - 1, adjustment % NANOSECONDS_IN_SECOND + NANOSECONDS_IN_SECOND)
        };

        prop_assert_eq!(expected_nanos as u32, instant.nano());
        prop_assert_eq!(expected_seconds, instant.epoch_second());
    }
}

proptest! {
    #[test]
    fn of_epoch_second_and_adjustment(seconds in prop::num::i64::ANY, nanos in 0..NANOSECONDS_IN_SECOND) {
        let instant = Instant::of_epoch_second_and_adjustment(seconds, nanos);

        prop_assert_eq!(nanos, instant.nano() as i64);
        prop_assert_eq!(seconds, instant.epoch_second());
    }
}

proptest! {
    #[test]
    #[should_panic(expected = "seconds would overflow instant")]
    fn of_epoch_second_and_adjustment_overflow(seconds in Just(i64::MAX), nanoseconds in Just(NANOSECONDS_IN_SECOND + 1)) {
        let _instant = Instant::of_epoch_second_and_adjustment(seconds, nanoseconds);
    }
}

proptest! {
    #[test]
    #[should_panic(expected = "seconds would overflow instant")]
    fn of_epoch_second_and_adjustment_underflow(seconds in Just(i64::MIN), nanoseconds in Just(-1)) {
        let _instant = Instant::of_epoch_second_and_adjustment(seconds, nanoseconds);
    }
}

proptest! {
    #[test]
    fn of_epoch_milli(millis in prop::num::i64::ANY) {
        let instant = Instant::of_epoch_milli(millis);

        let (seconds, nanos) = if millis >= 0 {
            (millis / MILLISECONDS_IN_SECOND, millis % MILLISECONDS_IN_SECOND * NANOSECONDS_IN_MILLISECOND)
        } else if millis % MILLISECONDS_IN_SECOND == 0 {
            (millis / MILLISECONDS_IN_SECOND, 0)
        } else {
            (millis / MILLISECONDS_IN_SECOND - 1, millis % MILLISECONDS_IN_SECOND * NANOSECONDS_IN_MILLISECOND + NANOSECONDS_IN_SECOND)
        };

        prop_assert_eq!(nanos as u32, instant.nano());
        prop_assert_eq!(seconds, instant.epoch_second());
    }
}
