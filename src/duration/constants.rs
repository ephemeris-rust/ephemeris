use crate::{constants::*, Duration};

#[test]
fn max() {
    let duration = Duration::MAX;
    assert_eq!(i64::MAX, duration.seconds());
    assert_eq!(NANOSECONDS_IN_SECOND as u32 - 1, duration.nano());
}

#[test]
fn zero() {
    let duration = Duration::ZERO;
    assert_eq!(0i64, duration.seconds());
    assert_eq!(0u32, duration.nano());
}

#[test]
fn min() {
    let duration = Duration::MIN;
    assert_eq!(i64::MIN, duration.seconds());
    assert_eq!(0u32, duration.nano());
}
