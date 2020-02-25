use std::i64;

use proptest::prelude::*;

use crate::Duration;

use crate::assert::expect_panic;

use crate::duration::test_util::*;

proptest! {
    #[test]
    fn sub_identity(duration in arb_duration()) {
        prop_assert_eq!(duration, duration - Duration::ZERO);
    }

    #[test]
    fn sub((left, _, total) in duration_total()) {
        let subtracted = next(total - left);
        let other = next(total) - left;
        prop_assert_eq!(subtracted, other);
    }

    #[test]
    fn sub_overflow((left, right) in arb_duration_overflow().prop_map(|(left, right)| (left, -right))) {
        expect_panic("duration subtraction would overflow", || left - right)?;
    }

    #[test]
    fn sub_underflow((left, right) in arb_duration_underflow().prop_map(|(left, right)| (left, -right))) {
        expect_panic("duration subtraction would overflow", || left - right)?;
    }

    #[test]
    fn sub_specific((left, right, expected)
        in prop_oneof!(
            Just((Duration::of_seconds_and_adjustment(i64::MIN, 0), Duration::of_seconds_and_adjustment(i64::MIN + 1, 0), Duration::of_seconds_and_adjustment(-1, 0))),

            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-2, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-3, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-3, 0))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-4, 666666668))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-4, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-4, 666666666))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-4, 333333334))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-4, 1))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-5, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-6, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-7, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-7, 333333334))),

            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-2, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-3, 666666666))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-3, 1))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-3, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-4, 999999999))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-4, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-4, 333333334))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-4, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-5, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-6, 0))),
            Just((Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-7, 666666667))),

            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(1, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(1, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-2, 666666666))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-2, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-2, 1))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-2, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-3, 999999999))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-3, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-3, 333333334))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-3, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-4, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-5, 0))),
            Just((Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-6, 666666667))),

            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(2, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(2, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(1, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-1, 666666666))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-1, 1))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-2, 999999999))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-2, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-2, 333333334))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-2, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-3, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-4, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-5, 666666667))),

            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(2, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(1, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(0, 333333333))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-1, 666666668))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-1, 666666666))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 333333334))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-1, 1))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-2, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-3, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-4, 666666667))),
            Just((Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-4, 333333334))),

            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(3, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(2, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(1, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(0, 666666666))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(0, 1))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-1, 999999999))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 666666667))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-1, 333333334))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-2, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-3, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-4, 666666667))),

            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 666666666))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(2, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(1, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(0, 999999999))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 666666666))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(0, 333333334))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(0, 333333332))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-1, 666666667))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-2, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-3, 0))),

            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(4, 333333333))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(4, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(3, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(2, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(1, 666666666))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(1, 333333333))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(1, 1))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(1, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(0, 999999999))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 666666667))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(0, 333333334))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-2, 0))),
            Just((Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-3, 666666667))),

            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(5, 333333333))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(5, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(4, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(3, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(2, 666666666))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(2, 333333333))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(2, 1))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(2, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(1, 999999999))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(1, 666666667))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(1, 333333334))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(1, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 0))),
            Just((Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-2, 666666667))),

            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(6, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(6, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(5, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(4, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(3, 666666666))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(3, 1))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(2, 999999999))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(2, 666666667))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(2, 333333334))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(2, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(1, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 666666667))),

            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(6, 666666666))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(6, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(5, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(4, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(3, 999999999))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 666666666))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(3, 333333334))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(3, 333333332))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 0))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(2, 666666667))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(2, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(1, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 333333333))),
            Just((Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 0))),

            Just((Duration::of_seconds_and_adjustment(i64::MAX, 0), Duration::of_seconds_and_adjustment(i64::MAX, 0), Duration::of_seconds_and_adjustment(0, 0))),
        )) {
        prop_assert_eq!(left - right, expected);
    }
}
