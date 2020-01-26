use proptest::prelude::*;

use crate::Duration;

use crate::duration::neg::amplitude;
use crate::duration::test_util::arb_duration;

proptest! {
    #[test]
    fn abs_sign(duration in arb_duration().prop_filter("Duration cannot be minimum".to_owned(), |d| d != &Duration::MIN)) {
        let result = duration.abs();
        prop_assert!(result >= Duration::ZERO);
    }

    #[test]
    fn neg_amplitude(duration in arb_duration().prop_filter("Duration cannot be minimum".to_owned(), |d| d != &Duration::MIN)) {
        let result = duration.abs();
        prop_assert_eq!(amplitude(duration), amplitude(result));
    }
}

#[test]
#[should_panic(expected = "absolute value would overflow duration")]
fn abs_overflow() {
    Duration::MIN.abs();
}
