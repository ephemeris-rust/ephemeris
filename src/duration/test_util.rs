use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

pub fn arb_duration() -> impl Strategy<Value = Duration> {
    (any::<i64>(), 0..NANOSECONDS_IN_SECOND)
        .prop_map(|(seconds, nanos)| Duration::of_seconds_and_adjustment(seconds, nanos))
}
