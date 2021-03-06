pub const NANOSECONDS_IN_MILLISECOND: i64 = NANOSECONDS_IN_SECOND / MILLISECONDS_IN_SECOND;
pub const NANOSECONDS_IN_SECOND: i64 = 1_000_000_000;
pub const NANOSECONDS_IN_MINUTE: i64 = SECONDS_IN_MINUTE * NANOSECONDS_IN_SECOND;
pub const NANOSECONDS_IN_HOUR: i64 = MINUTES_IN_HOUR * NANOSECONDS_IN_MINUTE;
pub const NANOSECONDS_IN_DAY: i64 = HOURS_IN_DAY * NANOSECONDS_IN_HOUR;
pub const MILLISECONDS_IN_SECOND: i64 = 1_000;
pub const SECONDS_IN_MINUTE: i64 = 60;
pub const SECONDS_IN_HOUR: i64 = SECONDS_IN_MINUTE * MINUTES_IN_HOUR;
pub const SECONDS_IN_DAY: i64 = SECONDS_IN_HOUR * HOURS_IN_DAY;
pub const MINUTES_IN_HOUR: i64 = 60;
pub const MINUTES_IN_DAY: i64 = MINUTES_IN_HOUR * HOURS_IN_DAY;
pub const HOURS_IN_DAY: i64 = 24;

pub const MAX_INSTANT_YEAR: i64 = 1_000_000_000;
pub const DAYS_IN_YEAR: i64 = 365;
pub const DAYS_IN_LONG_YEAR: i64 = 366;
pub const YEARS_IN_LEAP_YEAR_CYCLE: i64 = 4;
pub const DAYS_IN_LEAP_YEAR_CYCLE: i64 =
    (YEARS_IN_LEAP_YEAR_CYCLE - 1) * DAYS_IN_YEAR + DAYS_IN_LONG_YEAR;
pub const YEARS_IN_LONG_LEAP_YEAR_CYCLE: i64 = YEARS_IN_CENTURY;
pub const DAYS_IN_LONG_LEAP_YEAR_CYCLE: i64 =
    (YEARS_IN_LONG_LEAP_YEAR_CYCLE / YEARS_IN_LEAP_YEAR_CYCLE) * DAYS_IN_LEAP_YEAR_CYCLE - 1;
pub const YEARS_IN_LEAP_YEAR_EPICYCLE: i64 = 4 * YEARS_IN_LONG_LEAP_YEAR_CYCLE;
pub const DAYS_IN_LEAP_YEAR_EPICYCLE: i64 =
    (YEARS_IN_LEAP_YEAR_EPICYCLE / YEARS_IN_LONG_LEAP_YEAR_CYCLE) * DAYS_IN_LONG_LEAP_YEAR_CYCLE
        + 1;
pub const EPOCH_OFFSET_FROM_ZERO_YEARS: i64 = 1970;
pub const EPOCH_OFFSET_FROM_ZERO_DAYS: i64 = (2000 / YEARS_IN_LEAP_YEAR_EPICYCLE)
    * DAYS_IN_LEAP_YEAR_EPICYCLE
    - (2000 - EPOCH_OFFSET_FROM_ZERO_YEARS) * DAYS_IN_YEAR
    // Leap years from 1970 to 2000:
    // 1972, 1976, 1980, 1984, 1988, 1992, 1996
    - 7;
pub const EPOCH_OFFSET_FROM_ZERO_SECONDS: i64 = EPOCH_OFFSET_FROM_ZERO_DAYS * SECONDS_IN_DAY;
pub const YEARS_IN_DECADE: i64 = 10;
pub const YEARS_IN_CENTURY: i64 = 100;
pub const YEARS_IN_MILLENNIUM: i64 = 1000;
pub const DAYS_IN_WEEK_ISO: i64 = 7;
