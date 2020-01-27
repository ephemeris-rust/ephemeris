use std::i64;
use std::ops::*;

pub fn units_in_type_range(unit: i64) -> RangeInclusive<i64> {
    i64::MIN / unit..=i64::MAX / unit
}
