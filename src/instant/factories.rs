use std::i64;

use crate::assert::expect_panic;
use proptest::prelude::*;

use crate::constants::*;

use crate::Instant;

const INSTANT_SAFE_UPPER_BOUND: i64 =
    Instant::MAX.epoch_second() - i64::MAX / NANOSECONDS_IN_SECOND + 0;
const INSTANT_SAFE_LOWER_BOUND: i64 =
    Instant::MIN.epoch_second() - i64::MIN / NANOSECONDS_IN_SECOND + 1;

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
            let upper = if seconds >= INSTANT_SAFE_UPPER_BOUND {
                (Instant::MAX.epoch_second() - seconds + 1) * NANOSECONDS_IN_SECOND - 1
            } else {
                i64::MAX
            };

            let lower = if seconds < INSTANT_SAFE_LOWER_BOUND {
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

prop_compose! {
    fn adjustment_overflow()
        (seconds in INSTANT_SAFE_UPPER_BOUND + 1..=Instant::MAX.epoch_second())
            (seconds in Just(seconds), adjustment in (Instant::MAX.epoch_second() - seconds + 1) * NANOSECONDS_IN_SECOND..=i64::MAX)-> (i64, i64)
            {
                (seconds, adjustment)
            }
}

proptest! {
    #[test]
    fn of_epoch_second_and_adjustment_overflow((seconds, nanoseconds) in adjustment_overflow()) {
        expect_panic("seconds would overflow instant", || Instant::of_epoch_second_and_adjustment(seconds, nanoseconds))?;
    }
}

prop_compose! {
    fn adjustment_underflow()
        (seconds in Instant::MIN.epoch_second()..INSTANT_SAFE_LOWER_BOUND)
            (seconds in Just(seconds), adjustment in i64::MIN..=(i64::MIN - seconds - 1) * NANOSECONDS_IN_SECOND)-> (i64, i64)
            {
                (seconds, adjustment)
            }
}

proptest! {
    #[test]
    fn of_epoch_second_and_adjustment_underflow((seconds, nanoseconds) in adjustment_underflow()) {
        expect_panic("seconds would overflow instant", || Instant::of_epoch_second_and_adjustment(seconds, nanoseconds))?;
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
