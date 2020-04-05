use std::i64;

use proptest::prelude::*;

use crate::Duration;

use crate::assert::expect_panic;

use crate::duration::test_util::*;

prop_compose! {
    fn add_specific_data()
        (data in proptest::sample::select(
            vec![
                (Duration::of_seconds_and_adjustment(i64::MIN, 0), Duration::of_seconds_and_adjustment(i64::MAX, 0), Duration::of_seconds_and_adjustment(-1, 0)),

                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-7, 333333334)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-7, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-6, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-5, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-4, 1)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-4, 333333334)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-4, 666666666)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-4, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-4, 666666668)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-3, 333333333)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-3, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-2, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 666666667)),
                (Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 0)),

                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-7, 666666667)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-6, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-5, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-4, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-4, 333333334)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-4, 666666667)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-4, 999999999)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-3, 1)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-3, 333333333)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-3, 666666666)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 333333333)),

                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-6, 666666667)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-5, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-4, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-3, 333333334)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-3, 666666667)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-3, 999999999)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-2, 1)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-2, 333333333)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-2, 666666666)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(1, 333333333)),

                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-5, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-4, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-2, 333333334)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-2, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-2, 999999999)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-1, 1)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 333333333)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(-1, 666666666)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(2, 333333333)),

                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-4, 333333334)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-4, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-3, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-2, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-1, 1)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 333333334)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-1, 666666666)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(-1, 666666668)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(0, 333333333)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(1, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(2, 666666667)),
                (Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 0)),

                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-4, 666666667)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-1, 333333334)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(-1, 666666667)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(-1, 999999999)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(0, 1)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(0, 666666666)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 333333333)),

                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-3, 0)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-3, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-2, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(-1, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(-1, 666666667)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(0, 333333332)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(0, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(0, 333333334)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 666666666)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(0, 999999999)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(1, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(2, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(3, 333333333)),
                (Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 666666666)),

                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-3, 666666667)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-2, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(0, 333333334)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(0, 666666667)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(0, 999999999)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(1, 1)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(1, 333333333)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(1, 666666666)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(4, 0)),
                (Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(4, 333333333)),

                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-2, 666666667)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(-1, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(1, 333333334)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(1, 666666667)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(1, 999999999)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(2, 1)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(2, 333333333)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(2, 666666666)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(4, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(5, 0)),
                (Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(5, 333333333)),

                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(-1, 666666667)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(1, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(2, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(2, 333333334)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(2, 666666667)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(2, 999999999)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(3, 1)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(3, 666666666)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(4, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(5, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(6, 0)),
                (Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(6, 333333333)),

                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-4, 666666667), Duration::of_seconds_and_adjustment(0, 0)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-3, 0), Duration::of_seconds_and_adjustment(0, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-2, 0), Duration::of_seconds_and_adjustment(1, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 0), Duration::of_seconds_and_adjustment(2, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 333333334), Duration::of_seconds_and_adjustment(2, 666666667)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 666666667), Duration::of_seconds_and_adjustment(3, 0)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(-1, 999999999), Duration::of_seconds_and_adjustment(3, 333333332)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 0), Duration::of_seconds_and_adjustment(3, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 1), Duration::of_seconds_and_adjustment(3, 333333334)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 333333333), Duration::of_seconds_and_adjustment(3, 666666666)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(0, 666666666), Duration::of_seconds_and_adjustment(3, 999999999)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(1, 0), Duration::of_seconds_and_adjustment(4, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(2, 0), Duration::of_seconds_and_adjustment(5, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 0), Duration::of_seconds_and_adjustment(6, 333333333)),
                (Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(3, 333333333), Duration::of_seconds_and_adjustment(6, 666666666)),

                (Duration::of_seconds_and_adjustment(i64::MAX, 0), Duration::of_seconds_and_adjustment(i64::MIN, 0), Duration::of_seconds_and_adjustment(-1, 0)),
            ]
        )) -> (Duration, Duration, Duration)
        {
            data
        }
}

proptest! {
    #[test]
    fn add_unit(duration in arb_duration()) {
        prop_assert_eq!(duration, duration + Duration::ZERO);
    }

    #[test]
    fn add((left, right, _) in duration_total()) {
        let added = next(left + right);
        let other = next(left) + right;
        prop_assert_eq!(added, other);
    }

    #[test]
    fn add_overflow((left, right) in arb_duration_overflow()) {
        expect_panic("duration addition would overflow", || left + right)?;
    }

    #[test]
    fn add_underflow((left, right) in arb_duration_underflow()) {
        expect_panic("duration addition would overflow", || left + right)?;
    }

    #[test]
    fn add_specific((left, right, expected) in add_specific_data()) {
        prop_assert_eq!(left + right, expected);
    }
    #[test]
    fn add_assign_unit(duration in arb_duration()) {
        let mut other = duration;
        other += Duration::ZERO;

        prop_assert_eq!(duration, other);
    }

    #[test]
    fn add_assign((left, right, _) in duration_total()) {
        let mut left_a = left;
        let mut left_b = next(left);

        left_a += right;
        left_b += right;

        prop_assert_eq!(next(left_a), left_b);
    }

    #[test]
    fn add_assign_overflow((left, right) in arb_duration_overflow()) {
        expect_panic("duration addition would overflow", ||
        {
            let mut left_copy = left;
            left_copy += right;
        })?;
    }

    #[test]
    fn add_assign_underflow((left, right) in arb_duration_underflow()) {
        expect_panic("duration addition would overflow", || {
            let mut left_copy = left;
            left_copy += right;
        })?;
    }

    #[test]
    fn add_assign_specific((left, right, expected) in add_specific_data()) {
        let mut left_copy = left;
        left_copy += right;
        prop_assert_eq!(left_copy, expected);
    }
}
