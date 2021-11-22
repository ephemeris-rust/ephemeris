use std::ops::RangeInclusive;

pub(crate) const fn units_in_type_range(unit: i64) -> RangeInclusive<i64> {
    i64::MIN / unit..=i64::MAX / unit
}
