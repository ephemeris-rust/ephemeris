use std::fmt;
use std::i64;
use std::u32;

use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

use crate::constants::*;
use crate::seconds_nanos::*;
use crate::util::const_expect;

#[cfg(test)]
pub mod abs;
#[cfg(test)]
pub mod add;
#[cfg(test)]
pub mod add_assign;
#[cfg(test)]
pub mod constants;
#[cfg(test)]
pub mod display;
#[cfg(test)]
pub mod factories;
#[cfg(test)]
pub mod minus_unit;
#[cfg(test)]
pub mod neg;
#[cfg(test)]
pub mod plus_unit;
#[cfg(test)]
pub mod sub;
#[cfg(test)]
pub mod sub_assign;
#[cfg(test)]
pub mod test_util;
#[cfg(test)]
pub mod to;

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
    pub const fn of_days(days: i64) -> Duration {
        const_expect!(
            Duration::of_days_checked(days),
            "days would overflow duration"
        )
    }

    const fn of_days_checked(days: i64) -> Option<Duration> {
        // TODO: switch back to map when constant is stable
        match days.checked_mul(SECONDS_IN_DAY) {
            None => None,
            Some(total_seconds) => Some(Duration::of_seconds(total_seconds)),
        }
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
    pub const fn of_hours(hours: i64) -> Duration {
        const_expect!(
            Duration::of_hours_checked(hours),
            "hours would overflow duration"
        )
    }

    const fn of_hours_checked(hours: i64) -> Option<Duration> {
        // TODO: switch back to map when constant is stable
        match hours.checked_mul(SECONDS_IN_HOUR) {
            None => None,
            Some(total_seconds) => Some(Duration::of_seconds(total_seconds)),
        }
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
    pub const fn of_minutes(minutes: i64) -> Duration {
        const_expect!(
            Duration::of_minutes_checked(minutes),
            "minutes would overflow duration"
        )
    }

    const fn of_minutes_checked(minutes: i64) -> Option<Duration> {
        // TODO: switch back to map when constant is stable
        match minutes.checked_mul(SECONDS_IN_MINUTE) {
            None => None,
            Some(total_seconds) => Some(Duration::of_seconds(total_seconds)),
        }
    }

    /// Obtains a Duration representing a number of seconds and an adjustment in nanoseconds.
    ///
    /// # Parameters
    ///  - `seconds`: the seconds in the duration.
    ///  - `nano_adjustment`: the adjustment amount from the given second.
    ///
    /// # Panics
    /// - if the adjusted amount of seconds would overflow the duration.
    pub const fn of_seconds_and_adjustment(seconds: i64, nano_adjustment: i64) -> Duration {
        const_expect!(
            Duration::of_seconds_and_adjustment_checked(seconds, nano_adjustment),
            "nano adjustment would overflow duration"
        )
    }

    const fn of_seconds_and_adjustment_checked(
        seconds: i64,
        nano_adjustment: i64,
    ) -> Option<Duration> {
        // TODO: switch back to map when constant is stable
        match of_seconds_and_adjustment_checked(seconds, nano_adjustment) {
            None => None,
            Some((seconds, nanos)) => Some(Duration {
                seconds,
                nanoseconds_of_second: nanos,
            }),
        }
    }

    /// Obtains a Duration representing a number of seconds.
    ///
    /// The nanosecond field will be set to 0.
    ///
    /// # Parameters
    ///  - `seconds`: the seconds in the duration.
    pub const fn of_seconds(seconds: i64) -> Duration {
        Duration {
            seconds,
            nanoseconds_of_second: 0,
        }
    }

    /// Obtains a `Duration` representing a number of milliseconds.
    ///
    /// The seconds and nanoseconds are extracted from the specified milliseconds.
    ///
    /// # Parameters
    ///  - `millis`: the milliseconds in the duration.
    pub const fn of_millis(millis: i64) -> Duration {
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
    pub const fn of_nanos(nanoseconds: i64) -> Duration {
        let (seconds, nanos) = seconds_and_nanos(nanoseconds);
        Duration {
            seconds,
            nanoseconds_of_second: nanos,
        }
    }

    /// Gets the number of nanoseconds within the second in this duration.
    ///
    /// [`seconds()`]: struct.Duration.html#method.seconds
    pub const fn nano(&self) -> u32 {
        self.nanoseconds_of_second
    }

    /// Gets the number of seconds in this duration.
    ///
    /// [`nano()`]: struct.Duration.html#method.nano
    pub const fn seconds(&self) -> i64 {
        self.seconds
    }

    /// Returns a new duration with a positive length and matching amplitude.
    ///
    /// This method returns a positive duration by effectively removing the sign from any negative total length.
    /// For example, `PT-1.3S` will be returned as `PT1.3S`.
    ///
    /// # Panics
    /// - if the duration would overflow after removing the negative sign.
    pub fn abs(self) -> Duration {
        if self >= Duration::ZERO {
            self
        } else {
            checked_neg(self).expect("absolute value would overflow duration")
        }
    }

    /// Subtracts the specified number of days and returns the result as a new duration.
    ///
    /// The number of days is multiplied by 86,400 to obtain the number of seconds to subtract.
    /// This is based on the standard definition of a day as 24 hours.
    ///
    /// # Parameters
    ///  - `days`: the days to subtract, positive or negative.
    ///
    /// # Panics
    /// - if the subtraction would overflow the result duration.
    pub fn minus_days(self, days: i64) -> Duration {
        match days {
            i64::MIN => self
                .plus_days_checked(1)
                .and_then(|d| d.plus_days_checked(i64::MAX)),
            _ => self.plus_days_checked(-days),
        }
        .expect("subtraction of days overflowed")
    }

    /// Subtracts the specified number of hours and returns the result as a new duration.
    ///
    /// The number of hours is multiplied by 3,600 to obtain the number of seconds to subtract.
    /// This is based on the standard definition of an hour as 60 minutes.
    ///
    /// # Parameters
    ///  - `hours`: the hours to subtract, positive or negative.
    ///
    /// # Panics
    /// - if the subtraction would overflow the result duration.
    pub fn minus_hours(self, hours: i64) -> Duration {
        match hours {
            i64::MIN => self
                .plus_hours_checked(1)
                .and_then(|d| d.plus_hours_checked(i64::MAX)),
            _ => self.plus_hours_checked(-hours),
        }
        .expect("subtraction of hours overflowed")
    }

    /// Subtracts the specified number of minutes and returns the result as a new duration.
    ///
    /// The number of minutes is multiplied by 60 to obtain the number of seconds to subtract.
    /// This is based on the standard definition of a minute as 60 seconds.
    ///
    /// # Parameters
    ///  - `minutes`: the minutes to subtract, positive or negative.
    ///
    /// # Panics
    /// - if the subtraction would overflow the result duration.
    pub fn minus_minutes(self, minutes: i64) -> Duration {
        match minutes {
            i64::MIN => self
                .plus_minutes_checked(1)
                .and_then(|d| d.plus_minutes_checked(i64::MAX)),
            _ => self.plus_minutes_checked(-minutes),
        }
        .expect("subtraction of minutes overflowed")
    }

    /// Subtracts the specified number of seconds and returns the result as a new duration.
    ///
    /// # Parameters
    ///  - `seconds`: the second to subtract, positive or negative.
    ///
    /// # Panics
    /// - if the subtraction would overflow the result duration.
    pub fn minus_seconds(self, seconds: i64) -> Duration {
        match seconds {
            i64::MIN => self
                .plus_seconds_checked(1)
                .and_then(|d| d.plus_seconds_checked(i64::MAX)),
            _ => self.plus_seconds_checked(-seconds),
        }
        .expect("subtraction of seconds overflowed")
    }

    /// Subtracts the specified number of milliseconds and returns the result as a new duration.
    ///
    /// # Parameters
    ///  - `millis`: the milliseconds to subtract, positive or negative.
    ///
    /// # Panics
    /// - if the subtraction would overflow the result duration.
    pub fn minus_millis(self, millis: i64) -> Duration {
        match millis {
            i64::MIN => self
                .plus_millis_checked(1)
                .and_then(|d| d.plus_millis_checked(i64::MAX)),
            _ => self.plus_millis_checked(-millis),
        }
        .expect("subtraction of millis overflowed")
    }

    /// Subtracts the specified number of nanoseconds and returns the result as a new duration.
    ///
    /// # Parameters
    ///  - `nanos`: the nanoseconds to subtract, positive or negative.
    ///
    /// # Panics
    /// - if the subtraction would overflow the result duration.
    pub fn minus_nanos(self, nanos: i64) -> Duration {
        match nanos {
            i64::MIN => self
                .plus_nanos_checked(1)
                .and_then(|d| d.plus_nanos_checked(i64::MAX)),
            _ => self.plus_nanos_checked(-nanos),
        }
        .expect("subtraction of nanos overflowed")
    }

    /// Adds the specified number of days and returns the result as a new duration.
    ///
    /// The number of days is multiplied by 86,400 to obtain the number of seconds to add.
    /// This is based on the standard definition of a day as 24 hours.
    ///
    /// # Parameters
    ///  - `days`: the days to add, positive or negative.
    ///
    /// # Panics
    /// - if the addition would overflow the result duration.
    pub fn plus_days(self, days: i64) -> Duration {
        self.plus_days_checked(days)
            .expect("addition of days overflowed")
    }

    fn plus_days_checked(self, days: i64) -> Option<Duration> {
        match (self, days) {
            (_, 0) => Some(self),
            (Duration::ZERO, _) => Duration::of_days_checked(days),
            _ => days.checked_mul(SECONDS_IN_DAY).and_then(|total_seconds| {
                plus_internal(self.seconds(), total_seconds, self.nano() as i64)
            }),
        }
    }

    /// Adds the specified number of hours and returns the result as a new duration.
    ///
    /// The number of hours is multiplied by 3,600 to obtain the number of seconds to add.
    /// This is based on the standard definition of an hour as 60 minutes.
    ///
    /// # Parameters
    ///  - `hours`: the hours to add, positive or negative.
    ///
    /// # Panics
    /// - if the addition would overflow the result duration.
    pub fn plus_hours(self, hours: i64) -> Duration {
        self.plus_hours_checked(hours)
            .expect("addition of hours overflowed")
    }

    fn plus_hours_checked(self, hours: i64) -> Option<Duration> {
        match (self, hours) {
            (_, 0) => Some(self),
            (Duration::ZERO, _) => Duration::of_hours_checked(hours),
            _ => hours
                .checked_mul(SECONDS_IN_HOUR)
                .and_then(|total_seconds| {
                    plus_internal(self.seconds(), total_seconds, self.nano() as i64)
                }),
        }
    }

    /// Adds the specified number of minutes and returns the result as a new duration.
    ///
    /// The number of minutes is multiplied by 60 to obtain the number of seconds to add.
    /// This is based on the standard definition of a minute as 60 seconds.
    ///
    /// # Parameters
    ///  - `minutes`: the minutes to add, positive or negative.
    ///
    /// # Panics
    /// - if the addition would overflow the result duration.
    pub fn plus_minutes(self, minutes: i64) -> Duration {
        self.plus_minutes_checked(minutes)
            .expect("addition of minutes overflowed")
    }

    fn plus_minutes_checked(self, minutes: i64) -> Option<Duration> {
        match (self, minutes) {
            (_, 0) => Some(self),
            (Duration::ZERO, _) => Duration::of_minutes_checked(minutes),
            _ => minutes
                .checked_mul(SECONDS_IN_MINUTE)
                .and_then(|total_seconds| {
                    plus_internal(self.seconds(), total_seconds, self.nano() as i64)
                }),
        }
    }

    /// Adds the specified number of seconds and returns the result as a new duration.
    ///
    /// # Parameters
    ///  - `seconds`: the second to add, positive or negative.
    ///
    /// # Panics
    /// - if the addition would overflow the result duration.
    pub fn plus_seconds(self, seconds: i64) -> Duration {
        self.plus_seconds_checked(seconds)
            .expect("addition of seconds overflowed")
    }

    fn plus_seconds_checked(self, seconds: i64) -> Option<Duration> {
        match (self, seconds) {
            (_, 0) => Some(self),
            (Duration::ZERO, _) => Some(Duration::of_seconds(seconds)),
            _ => plus_internal(self.seconds(), seconds, self.nano() as i64),
        }
    }

    /// Adds the specified number of milliseconds and returns the result as a new duration.
    ///
    /// # Parameters
    ///  - `millis`: the milliseconds to add, positive or negative.
    ///
    /// # Panics
    /// - if the addition would overflow the result duration.
    pub fn plus_millis(self, millis: i64) -> Duration {
        self.plus_millis_checked(millis)
            .expect("addition of millis overflowed")
    }

    fn plus_millis_checked(self, millis: i64) -> Option<Duration> {
        match (self, millis) {
            (_, 0) => Some(self),
            (Duration::ZERO, _) => Some(Duration::of_millis(millis)),
            _ => {
                let other_seconds = millis / MILLISECONDS_IN_SECOND;
                let other_nanos = (millis % MILLISECONDS_IN_SECOND) * NANOSECONDS_IN_MILLISECOND;
                plus_internal(
                    self.seconds(),
                    other_seconds,
                    self.nano() as i64 + other_nanos,
                )
            }
        }
    }

    /// Adds the specified number of nanoseconds and returns the result as a new duration.
    ///
    /// # Parameters
    ///  - `nanos`: the nanoseconds to add, positive or negative.
    ///
    /// # Panics
    /// - if the addition would overflow the result duration.
    pub fn plus_nanos(self, nanos: i64) -> Duration {
        self.plus_nanos_checked(nanos)
            .expect("addition of nanos overflowed")
    }

    fn plus_nanos_checked(self, nanos: i64) -> Option<Duration> {
        match (self, nanos) {
            (_, 0) => Some(self),
            (Duration::ZERO, _) => Some(Duration::of_nanos(nanos)),
            _ => {
                let other_seconds = nanos / NANOSECONDS_IN_SECOND;
                let other_nanos = nanos % NANOSECONDS_IN_SECOND;
                plus_internal(
                    self.seconds(),
                    other_seconds,
                    self.nano() as i64 + other_nanos,
                )
            }
        }
    }

    /// The total number of days in the duration.
    ///
    /// This returns the total number of days in the duration by dividing the number of seconds by 86,400.
    /// This is based on the standard definition of a day as 24 hours.
    pub fn to_days(&self) -> i64 {
        self.seconds() / SECONDS_IN_DAY
    }

    /// The total number of hours in the duration.
    ///
    /// This returns the total number of hours in the duration by dividing the number of seconds by 3,600.
    /// This is based on the standard definition of an hour as 60 minutes.
    pub fn to_hours(&self) -> i64 {
        self.seconds() / SECONDS_IN_HOUR
    }

    /// The total number of minutes in the duration.
    ///
    /// This returns the total number of minutes in the duration by dividing the number of seconds by 60.
    /// This is based on the standard definition of a minute as 60 seconds.
    pub fn to_minutes(&self) -> i64 {
        self.seconds() / SECONDS_IN_MINUTE
    }

    /// The total number of milliseconds in the duration.
    ///
    /// This returns the total number of milliseconds in the duration by multiplying the number of seconds by 1,000.
    ///
    /// # Panics
    /// - if the amount of milliseconds would overflow an i64.
    pub fn to_millis(&self) -> i64 {
        self.seconds()
            .checked_mul(MILLISECONDS_IN_SECOND)
            .and_then(|result| result.checked_add(self.nano() as i64 / NANOSECONDS_IN_MILLISECOND))
            .expect("total milliseconds would overflow")
    }

    /// The total number of nanoseconds in the duration.
    ///
    /// This returns the total number of nanoseconds in the duration by multiplying the number of seconds by 1,000,000,000.
    ///
    /// # Panics
    /// - if the amount of nanoseconds would overflow an i64.
    pub fn to_nanos(&self) -> i64 {
        self.seconds()
            .checked_mul(NANOSECONDS_IN_SECOND)
            .and_then(|result| result.checked_add(self.nano() as i64))
            .expect("total nanoseconds would overflow")
    }
}

impl Add for Duration {
    type Output = Duration;

    /// Adds one duration to another and returns the result as a new duration.
    ///
    /// # Parameters
    /// - `rhs`: the other duration to add, positive or negative.
    ///
    /// # Panics
    ///  - if the addition would overflow the duration.
    fn add(self, rhs: Duration) -> Duration {
        plus_internal(
            self.seconds(),
            rhs.seconds(),
            self.nano() as i64 + rhs.nano() as i64,
        )
        .expect("duration addition would overflow")
    }
}

// This method assumes `left_nanos + right_nanos` will be within one step of correct nanoseconds.
// (Mostly this means Durations must be well-formed before adding)
fn plus_internal(left_seconds: i64, right_seconds: i64, nanos: i64) -> Option<Duration> {
    let (adjustment, final_nanos) = carry_and_nanos(nanos);

    right_seconds
        .checked_add(adjustment)
        .and_then(|adjusted_seconds| left_seconds.checked_add(adjusted_seconds))
        .map(|final_seconds| Duration {
            seconds: final_seconds,
            nanoseconds_of_second: final_nanos,
        })
}

impl AddAssign for Duration {
    /// Adds one duration to another, and assigns the result to the left hand operand.
    ///
    /// # Parameters
    /// - `rhs`: the other duration to add, positive or negative.
    ///
    /// # Panics
    ///  - if the addition would overflow the duration.
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl fmt::Display for Duration {
    /// A string representation of this duration using ISO-8601 seconds based representation,
    /// such as PT8H6M12.345S.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self == &Duration::ZERO {
            return f.write_str("PT0S");
        }

        f.write_str("PT")?;

        let (effective_seconds, directed_nanos) = if self.seconds() >= 0 || self.nano() == 0 {
            (self.seconds(), self.nano())
        } else {
            (
                self.seconds() + 1,
                NANOSECONDS_IN_SECOND as u32 - self.nano(),
            )
        };

        let hours = effective_seconds / SECONDS_IN_HOUR;
        if hours != 0 {
            hours.fmt(f)?;
            f.write_str("H")?;
        }

        let minutes = (effective_seconds % SECONDS_IN_HOUR) / SECONDS_IN_MINUTE;
        if minutes != 0 {
            minutes.fmt(f)?;
            f.write_str("M")?;
        }

        let remaining_seconds = effective_seconds % SECONDS_IN_MINUTE;
        if remaining_seconds != 0 || directed_nanos != 0 {
            if remaining_seconds == 0 && effective_seconds < 0 {
                f.write_str("-0")?;
            } else {
                remaining_seconds.fmt(f)?;
            }

            if directed_nanos != 0 {
                let formatted = format!(".{:09}", directed_nanos);
                f.write_str(formatted.as_str().trim_end_matches('0'))?;
            }
            f.write_str("S")?;
        }

        Ok(())
    }
}

impl Neg for Duration {
    type Output = Duration;

    /// Returns a copy of this duration with the length negated.
    ///
    /// This method swaps the sign of the total length of this duration.
    /// For example, `PT1.3S` will be returned as `PT-1.3S`.
    ///
    /// # Panics
    ///  - if swapping the sign would cause the duration to overflow.
    fn neg(self) -> Duration {
        checked_neg(self).expect("negated value would overflow duration")
    }
}

fn checked_neg(duration: Duration) -> Option<Duration> {
    match (duration.seconds(), duration.nano()) {
        (i64::MIN, 0) => None,
        (i64::MIN, nanos) => Some(Duration {
            seconds: i64::MAX,
            nanoseconds_of_second: NANOSECONDS_IN_SECOND as u32 - nanos,
        }),
        (seconds, nanos) => {
            let (adjustment, flipped_nanos) = carry_and_nanos(-(nanos as i64));
            Some(Duration {
                seconds: -seconds + adjustment,
                nanoseconds_of_second: flipped_nanos,
            })
        }
    }
}

impl Sub for Duration {
    type Output = Duration;

    /// Subtracts one duration from another and returns the result as a new duration.
    ///
    /// # Parameters
    /// - `rhs`: the other duration to subtract, positive or negative.
    ///
    /// # Panics
    ///  - if the subtraction would overflow the duration.
    fn sub(self, rhs: Duration) -> Duration {
        match (rhs, rhs.seconds()) {
            (Duration::ZERO, _) => Some(self),
            (_, i64::MIN) =>
            // Do the offsetting of nanos early, because otherwise nano-offset + max seconds might overflow.
            {
                plus_internal(self.seconds(), 1, self.nano() as i64 - (rhs.nano() as i64))
                    .and_then(|added| plus_internal(added.seconds(), i64::MAX, added.nano() as i64))
            }
            _ => plus_internal(
                self.seconds(),
                -rhs.seconds(),
                self.nano() as i64 - (rhs.nano() as i64),
            ),
        }
        .expect("duration subtraction would overflow")
    }
}

impl SubAssign for Duration {
    /// Subtracts one duration from another, and assigns the result to the left hand operand.
    ///
    /// # Parameters
    /// - `rhs`: the other duration to subtract, positive or negative.
    ///
    /// # Panics
    ///  - if the subtraction would overflow the duration.
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}
