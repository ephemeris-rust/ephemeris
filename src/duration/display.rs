use std::i64;

use proptest::prelude::*;
use proptest::sample::*;

use crate::constants::*;

use crate::Duration;

prop_compose! {
    fn duration_and_format()
        (sign in select(vec![-1, 1]),
         hours in 0..i64::MAX / SECONDS_IN_HOUR,
         minutes in 0..MINUTES_IN_HOUR,
         seconds in 0..SECONDS_IN_MINUTE,
         nanos in 0..NANOSECONDS_IN_SECOND) -> (Duration, String)
        {
            if hours == 0 && minutes == 0 && seconds == 0 && nanos == 0 {
                return (Duration::ZERO, "PT0S".to_owned());
            }

            let duration = Duration::of_seconds_and_adjustment((hours * SECONDS_IN_HOUR + minutes * SECONDS_IN_MINUTE + seconds) * sign, nanos * sign);

            let mut formatted = String::from("PT");
            if hours > 0 {
                formatted += format!("{}H", hours * sign).as_str();
            }
            if minutes > 0 {
                formatted += format!("{}M", minutes * sign).as_str();
            }

            if seconds > 0 || nanos > 0 {
                if seconds == 0 && sign == -1 {
                    formatted += "-0";
                } else  {
                    formatted += format!("{}", seconds * sign).as_str();
                }
                if nanos > 0 {
                    formatted += format!(".{:09}", nanos).as_str().trim_end_matches('0');
                }
                formatted += "S";
            }

            (duration, formatted)
        }
}

proptest! {
    #[test]
    fn display((duration, ref expected) in duration_and_format()) {
        let formatted = format!("{}", duration);
        prop_assert_eq!(expected, &formatted);
    }
}

#[test]
fn display_zero() {
    assert_eq!("PT0S", format!("{}", Duration::ZERO));
}
