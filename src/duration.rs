use std;
use std::i64;
use std::u32;

use crate::constants::*;
use crate::seconds_nanos::*;

#[cfg(test)]
pub mod factories;

/// A time-based amount of time, such as '34.5 seconds'.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
    seconds: i64,
    nanoseconds_of_second: u32,
}

impl Duration {
    /// Constant for a duration with the greatest negative length.
    pub const MIN: Duration = Duration {
        seconds: i64::MIN,
        nanoseconds_of_second: 0,
    };
    /// Constant for a duration of zero length.
    pub const ZERO: Duration = Duration {
        seconds: 0,
        nanoseconds_of_second: 0,
    };
    /// Constant for a duration with the greatest positive length.
    pub const MAX: Duration = Duration {
        seconds: i64::MAX,
        nanoseconds_of_second: NANOSECONDS_IN_SECOND as u32 - 1,
    };

    /// Obtains a `Duration` representing a number of standard days.
    ///
    /// The seconds are calculated based on the standard definition of a day, where each day is 86,400 seconds.
    /// The nanosecond in second field is set to zero.
    ///
    /// # Parameters
    ///  - `days`: the days in the duration.
    ///
    /// # Panics
    /// - if the amount of days would overflow the duration.
    pub fn of_days(days: i64) -> Duration {
        Duration::of_days_checked(days).expect("days would overflow duration")
    }

    fn of_days_checked(days: i64) -> Option<Duration> {
        days.checked_mul(SECONDS_IN_DAY)
            .map(|total_seconds| Duration::of_seconds(total_seconds))
    }

    /// Obtains a `Duration` representing a number of standard hours.
    ///
    /// The seconds are calculated based on the standard definition of an hour, where each hour is 3600 seconds.
    /// The nanosecond in second field is set to zero.
    ///
    /// # Parameters
    ///  - `hours`: the hours in the duration.
    ///
    /// # Panics
    /// - if the amount of hours would overflow the duration.
    pub fn of_hours(hours: i64) -> Duration {
        Duration::of_hours_checked(hours).expect("hours would overflow duration")
    }

    fn of_hours_checked(hours: i64) -> Option<Duration> {
        hours
            .checked_mul(SECONDS_IN_HOUR)
            .map(|total_seconds| Duration::of_seconds(total_seconds))
    }

    /// Obtains a `Duration` representing a number of standard minutes.
    ///
    /// The seconds are calculated based on the standard definition of a minute, where each minute is 60 seconds.
    /// The nanosecond in second field is set to zero.
    ///
    /// # Parameters
    ///  - `minutes`: the minutes in the duration.
    ///
    /// # Panics
    /// - if the amount of minutes would overflow the duration.
    pub fn of_minutes(minutes: i64) -> Duration {
        Duration::of_minutes_checked(minutes).expect("minutes would overflow duration")
    }

    fn of_minutes_checked(minutes: i64) -> Option<Duration> {
        minutes
            .checked_mul(SECONDS_IN_MINUTE)
            .map(|total_seconds| Duration::of_seconds(total_seconds))
    }

    /// Obtains a Duration representing a number of seconds and an adjustment in nanoseconds.
    ///
    /// # Parameters
    ///  - `seconds`: the seconds in the duration.
    ///  - `nano_adjustment`: the adjustment amount from the given second.
    ///
    /// # Panics
    /// - if the adjusted amount of seconds would overflow the duration.
    pub fn of_seconds_and_adjustment(seconds: i64, nano_adjustment: i64) -> Duration {
        Duration::of_seconds_and_adjustment_checked(seconds, nano_adjustment)
            .expect("nano adjustment would overflow duration")
    }

    fn of_seconds_and_adjustment_checked(seconds: i64, nano_adjustment: i64) -> Option<Duration> {
        of_seconds_and_adjustment_checked(seconds, nano_adjustment).map(|(seconds, nanos)| {
            Duration {
                seconds: seconds,
                nanoseconds_of_second: nanos,
            }
        })
    }

    /// Obtains a Duration representing a number of seconds.
    ///
    /// The nanosecond field will be set to 0.
    ///
    /// # Parameters
    ///  - `seconds`: the seconds in the duration.
    pub const fn of_seconds(seconds: i64) -> Duration {
        Duration {
            seconds: seconds,
            nanoseconds_of_second: 0,
        }
    }

    /// Obtains a `Duration` representing a number of milliseconds.
    ///
    /// The seconds and nanoseconds are extracted from the specified milliseconds.
    ///
    /// # Parameters
    ///  - `millis`: the milliseconds in the duration.
    pub fn of_millis(millis: i64) -> Duration {
        let seconds = millis / MILLISECONDS_IN_SECOND;
        let adjustment = (millis % MILLISECONDS_IN_SECOND) * NANOSECONDS_IN_MILLISECOND;

        let (second_adjustment, nanos) = carry_and_nanos(adjustment);

        Duration {
            seconds: seconds + second_adjustment,
            nanoseconds_of_second: nanos,
        }
    }

    /// Obtains a `Duration` representing a number of nanoseconds.
    ///
    /// The seconds and nanoseconds are extracted from the specified nanoseconds.
    ///
    /// # Parameters
    ///  - `nanos`: the nanos in the duration.
    pub fn of_nanos(nanoseconds: i64) -> Duration {
        let (seconds, nanos) = seconds_and_nanos(nanoseconds);
        Duration {
            seconds: seconds,
            nanoseconds_of_second: nanos,
        }
    }

    /// Gets the number of nanoseconds within the second in this duration.
    ///
    /// [`seconds()`]: struct.Duration.html#method.seconds
    pub const fn nanos(&self) -> u32 {
        self.nanoseconds_of_second
    }

    /// Gets the number of seconds in this duration.
    ///
    /// [`nanos()`]: struct.Duration.html#method.nanos
    pub const fn seconds(&self) -> i64 {
        self.seconds
    }
}
