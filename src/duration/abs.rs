use proptest::prelude::*;

pub(crate) use crate::{
    duration::{neg::amplitude, test_util::*},
    Duration,
};

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
