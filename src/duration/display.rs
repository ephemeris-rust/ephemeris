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

prop_compose! {
    fn display_specific_data()
        (data in proptest::sample::select(
            vec![
                (Duration::of_seconds_and_adjustment(0, 0), "PT0S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 1), "PT0.000000001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 10), "PT0.00000001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 100), "PT0.0000001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 1000), "PT0.000001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 10000), "PT0.00001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 100000), "PT0.0001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 1000000), "PT0.001S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 10000000), "PT0.01S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 100000000), "PT0.1S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 120000000), "PT0.12S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123000000), "PT0.123S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123400000), "PT0.1234S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123450000), "PT0.12345S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123456000), "PT0.123456S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123456700), "PT0.1234567S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123456780), "PT0.12345678S".to_owned()),
                (Duration::of_seconds_and_adjustment(0, 123456789), "PT0.123456789S".to_owned()),
                (Duration::of_seconds_and_adjustment(1, 0), "PT1S".to_owned()),
                (Duration::of_seconds_and_adjustment(59, 0), "PT59S".to_owned()),
                (Duration::of_seconds_and_adjustment(60, 0), "PT1M".to_owned()),
                (Duration::of_seconds_and_adjustment(61, 0), "PT1M1S".to_owned()),
                (Duration::of_seconds_and_adjustment(3599, 0), "PT59M59S".to_owned()),
                (Duration::of_seconds_and_adjustment(3600, 0), "PT1H".to_owned()),
                (Duration::of_seconds_and_adjustment(3601, 0), "PT1H1S".to_owned()),
                (Duration::of_seconds_and_adjustment(3661, 0), "PT1H1M1S".to_owned()),
                (Duration::of_seconds_and_adjustment(86399, 0), "PT23H59M59S".to_owned()),
                (Duration::of_seconds_and_adjustment(86400, 0), "PT24H".to_owned()),
                (Duration::of_seconds_and_adjustment(59, 0), "PT59S".to_owned()),
                (Duration::of_seconds_and_adjustment(59, 0), "PT59S".to_owned()),
                (Duration::of_seconds_and_adjustment(-1, 0), "PT-1S".to_owned()),
                (Duration::of_seconds_and_adjustment(-1, 1000), "PT-0.999999S".to_owned()),
                (Duration::of_seconds_and_adjustment(-1, 900000000), "PT-0.1S".to_owned()),
                (Duration::of_seconds_and_adjustment(-60, 100_000_000), "PT-59.9S".to_owned()),
                (Duration::of_seconds_and_adjustment(-59, -900_000_000), "PT-59.9S".to_owned()),
                (Duration::of_seconds_and_adjustment(-60, -100_000_000), "PT-1M-0.1S".to_owned()),
                (Duration::of_seconds_and_adjustment(i64::MAX, 0), format!("PT{}H{}M{}S", i64::MAX / 3600,  (i64::MAX % 3600) / 60, i64::MAX % 60)),
                (Duration::of_seconds_and_adjustment(i64::MIN, 0), format!("PT{}H{}M{}S", i64::MIN / 3600,  (i64::MIN % 3600) / 60, i64::MIN % 60)),
            ]
        )) -> (Duration, String)
        {
            data
        }
}

proptest! {
    #[test]
    fn display((duration, ref expected) in duration_and_format()) {
        let formatted = format!("{}", duration);
        prop_assert_eq!(expected, &formatted);
    }

    #[test]
    fn display_specific((duration, ref expected) in display_specific_data()) {
        let formatted = format!("{}", duration);
        prop_assert_eq!(expected, &formatted);
    }
}

#[test]
fn display_zero() {
    assert_eq!("PT0S", format!("{}", Duration::ZERO));
}
