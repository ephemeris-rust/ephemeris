use std::i64;

use proptest::prelude::*;

use crate::constants::*;

use crate::Duration;

use crate::duration::test_util::arb_duration;

proptest! {
    #[test]
    fn neg_sign(duration in arb_duration().prop_filter("Duration cannot be minimum".to_owned(), |d| d != &Duration::MIN)) {
        let negated = -duration;

        if duration == Duration::ZERO {
            prop_assert_eq!(duration, negated);
        } else {
            prop_assert_ne!(duration < Duration::ZERO, negated < Duration::ZERO);
        }
    }

    #[test]
    fn neg_amplitude(duration in arb_duration().prop_filter("Duration cannot be minimum".to_owned(), |d| d != &Duration::MIN)) {
        let negated = -duration;
        prop_assert_eq!(amplitude(duration), amplitude(negated));
    }
}

pub fn amplitude(duration: Duration) -> (i64, u32) {
    match (duration.seconds(), duration.nano()) {
        (0..=i64::MAX, _) => (duration.seconds(), duration.nano()),
        (seconds, 0) => (-seconds, 0),
        (seconds, nanos) => (-seconds - 1, NANOSECONDS_IN_SECOND as u32 - nanos),
    }
}

#[test]
#[should_panic(expected = "negated value would overflow duration")]
fn neg_overflow() {
    let _result = -Duration::MIN;
}
