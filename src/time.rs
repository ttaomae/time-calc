use std::fmt;

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal::prelude::ToPrimitive;

#[derive(Clone, Copy)]
struct Time {
    seconds: i64,
    nanoseconds: u32,
}

struct TimeBuilder {
    negative: bool,
    hours: u64,
    minutes: u8,
    seconds: u8,
    nanoseconds: u32,
}

impl TimeBuilder {
    fn new() -> TimeBuilder {
        TimeBuilder {
            negative: false,
            hours: 0,
            minutes: 0,
            seconds: 0,
            nanoseconds: 0,
        }
    }

    fn negative(&mut self) -> &mut TimeBuilder {
        self.negative  = true;
        self
    }

    fn hours(&mut self, hours: u64) -> &mut TimeBuilder {
        self.hours = hours;
        self
    }

    fn minutes(&mut self, minutes: u8) -> &mut TimeBuilder {
        self.minutes = minutes;
        self
    }

    fn seconds(&mut self, seconds: u8) -> &mut TimeBuilder {
        self.seconds = seconds;
        self
    }

    fn nanoseconds(&mut self, nanoseconds: u32) -> &mut TimeBuilder {
        self.nanoseconds = nanoseconds;
        self
    }

    fn build(&self) -> Time {
        let mut seconds = (self.hours as u64 * Time::SECONDS_PER_HOUR as u64
            + self.minutes as u64 * Time::SECONDS_PER_MINUTE as u64
            + self.seconds as u64) as i64;

        let mut nanoseconds = self.nanoseconds;

        // Since times are represented as a number of seconds plus a nanosecond offset, negative
        // numbers must be one less than the whole number of seconds. For example, -1.2 seconds is
        // represented as -2 seconds + .8 seconds (800,000,000 ns).
        // Exclude special case of -0.0s.
        if self.negative && (seconds != 0 || nanoseconds != 0) {
            seconds = -seconds - 1;
            nanoseconds = Time::NANOS_PER_SECOND - nanoseconds;
        }

        Time { seconds, nanoseconds: nanoseconds as u32 }
    }
}

impl Time {
    const NANOS_PER_SECOND: u32 = 1_000_000_000;
    const SECONDS_PER_MINUTE: u8 = 60;
    const MINUTES_PER_HOUR: u8 = 60;
    const SECONDS_PER_HOUR: u16 = Time::MINUTES_PER_HOUR as u16 * Time::SECONDS_PER_MINUTE as u16;

    fn total_seconds(self) -> i64 {
        if self.seconds < 0 && self.nanoseconds == Time::NANOS_PER_SECOND {
            self.seconds + 1
        }
        else {
            self.seconds
        }
    }

    fn signum(self) -> i64 {
        if self.seconds == 0 {
            (self.nanoseconds as i64).signum()
        }
        else {
            self.seconds.signum()
        }
    }

    fn hours(self) -> u64 {
        let mut total_seconds = self.seconds;
        if total_seconds < 0 {
            total_seconds += 1;
        }

        (total_seconds / Time::SECONDS_PER_HOUR as i64).abs() as u64
    }

    fn minutes(self) -> u8 {
        let mut total_seconds = self.seconds;
        if total_seconds < 0  {
            total_seconds += 1;
        }

        ((total_seconds % Time::SECONDS_PER_HOUR as i64) / Time::MINUTES_PER_HOUR as i64).abs() as u8
    }

    fn seconds(self) -> u8 {
        let mut total_seconds = self.seconds;
        if total_seconds < 0 {
            total_seconds += 1;
        }

        (total_seconds % Time::SECONDS_PER_MINUTE as i64).abs() as u8
    }

    fn nanoseconds(self) -> u32 {
        if self.seconds < 0 {
            Time::NANOS_PER_SECOND - self.nanoseconds
        }
        else {
            self.nanoseconds
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hours = self.hours();
        let minutes = self.minutes();
        let seconds = self.seconds();
        let nanoseconds = self.nanoseconds();

        if self.total_seconds() < 0 {
            write!(f, "-");
        }
        if hours > 0 {
            write!(f, "{}:", hours);
        }
        if hours > 0 || minutes > 0 {
            write!(f, "{:02}:{:02}", minutes, seconds);
        }
        else {
            write!(f, "{}", seconds);
        }

        if nanoseconds > 0 {
            let mut nanos = format!("{:09}", nanoseconds);
            // Remove trailing zeros.
            while let Some(c) = nanos.pop() {
                // Found non-'0'. Put it back and break from loop.
                if c != '0' {
                    nanos.push(c);
                    break;
                }
            }

            write!(f, ".{}", nanos);
        }
        if hours == 0 && minutes == 0 {
            write!(f, "s");
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Time;
    use crate::time::TimeBuilder;

    #[test]
    fn test_time_seconds_only() {
        // 00:00:00
        assert_time(TimeBuilder::new().build(),
                    0, 0, 0, 0, 0, 0, "0s");
        assert_time(TimeBuilder::new().negative().build(),
                    0, 0, 0, 0, 0, 0, "0s");

        // +/- 00:00:01
        assert_time(TimeBuilder::new().seconds(1).build(),
                    1, 1, 0, 0, 1, 0, "1s");
        assert_time(TimeBuilder::new().negative().seconds(1).build(),
                    -1, -1, 0, 0, 1, 0, "-1s");

        // +/- 00:01:00
        assert_time(TimeBuilder::new().minutes(1).build(),
                    60, 1, 0, 1, 0, 0, "01:00");
        assert_time(TimeBuilder::new().negative().minutes(1).build(),
                    -60, -1, 0, 1, 0, 0, "-01:00");

        // +/- 01:00:00
        let seconds = 60 * 60;
        assert_time(TimeBuilder::new().hours(1).build(),
                    seconds, 1, 1, 0, 0, 0, "1:00:00");
        assert_time(TimeBuilder::new().negative().hours(1).build(),
                    -seconds, -1, 1, 0, 0, 0, "-1:00:00");

        // +/- 00:01:01
        assert_time(TimeBuilder::new().minutes(1).seconds(1).build(),
                    61, 1, 0, 1, 1, 0, "01:01");
        assert_time(TimeBuilder::new().negative().minutes(1).seconds(1).build(),
                    -61, -1, 0, 1, 1, 0, "-01:01");

        // +/- 01:01:01
        let seconds = (60 * 60) + 60 + 1;
        assert_time(TimeBuilder::new().hours(1).minutes(1).seconds(1).build(),
                seconds, 1, 1, 1, 1, 0, "1:01:01");
        assert_time(TimeBuilder::new().negative().hours(1).minutes(1).seconds(1).build(),
                    -seconds, -1, 1, 1, 1, 0, "-1:01:01");

        // +/- 00:00:59
        assert_time(TimeBuilder::new().seconds(59).build(),
                    59, 1, 0, 0, 59, 0, "59s");
        assert_time(TimeBuilder::new().negative().seconds(59).build(),
                    -59, -1, 0, 0, 59, 0, "-59s");

        // +/- 00:59:59
        let seconds = (60 * 60) - 1;
        assert_time(TimeBuilder::new().minutes(59).seconds(59).build(),
                    seconds, 1, 0, 59, 59, 0, "59:59");
        assert_time(TimeBuilder::new().negative().minutes(59).seconds(59).build(),
                    -seconds, -1, 0, 59, 59, 0, "-59:59");
    }

    #[test]
    fn test_time_nanos_only() {
        // +/- 0.000000001s
        assert_time(TimeBuilder::new().nanoseconds(1).build(),
                    0, 1, 0, 0, 0, 1, "0.000000001s");
        assert_time(TimeBuilder::new().negative().nanoseconds(1).build(),
                    -1, -1, 0, 0, 0, 1, "-0.000000001s");

        // +/- 0.999999999s
        assert_time(TimeBuilder::new().nanoseconds(999999999).build(),
                    0, 1, 0, 0, 0, 999999999, "0.999999999s");
        assert_time(TimeBuilder::new().negative().nanoseconds(999999999).build(),
                    -1, -1, 0, 0, 0, 999999999, "-0.999999999s");

        // +/- 0.123456789
        assert_time(TimeBuilder::new().nanoseconds(123456789).build(),
                    0, 1, 0, 0 ,0, 123456789, "0.123456789s");
        assert_time(TimeBuilder::new().negative().nanoseconds(123456789).build(),
                    -1, -1, 0, 0, 0, 123456789, "-0.123456789s");

        // +/- 0.987654321
        assert_time(TimeBuilder::new().nanoseconds(987654321).build(),
                    0, 1, 0, 0, 0, 987654321, "0.987654321s");
        assert_time(TimeBuilder::new().negative().nanoseconds(987654321).build(),
                    -1, -1, 0, 0, 0, 987654321, "-0.987654321s");

        // +/- 0.13579
        assert_time(TimeBuilder::new().nanoseconds(135790000).build(),
                    0, 1, 0, 0, 0, 135790000, "0.13579s");
        assert_time(TimeBuilder::new().negative().nanoseconds(135790000).build(),
                    -1, -1, 0, 0, 0, 135790000, "-0.13579s");

        // +/- 0.2468
        assert_time(TimeBuilder::new().nanoseconds(246800000).build(),
                    0, 1, 0, 0, 0, 246800000, "0.2468s");
        assert_time(TimeBuilder::new().negative().nanoseconds(246800000).build(),
                    -1, -1, 0, 0, 0, 246800000, "-0.2468s");
    }

    #[test]
    fn test_time_seconds_and_nanos() {
        // +/- 00:00:01.000000001
        assert_time(TimeBuilder::new().seconds(1).nanoseconds(1).build(),
                    1, 1, 0, 0, 1, 1, "1.000000001s");
        assert_time(TimeBuilder::new().negative().seconds(1).nanoseconds(1).build(),
                    -2, -1, 0, 0, 1, 1, "-1.000000001s");

        // +/- 00:01:01.000000001
        assert_time(TimeBuilder::new().minutes(1).seconds(1).nanoseconds(1).build(),
                    61, 1, 0, 1, 1, 1, "01:01.000000001");
        assert_time(TimeBuilder::new().negative().minutes(1).seconds(1).nanoseconds(1).build(),
                    -62, -1, 0, 1, 1, 1, "-01:01.000000001");

        // +/- 01:01:01.000000001
        let seconds = (60 * 60) + 60 + 1;
        assert_time(TimeBuilder::new().hours(1).minutes(1).seconds(1).nanoseconds(1).build(),
                    seconds, 1, 1, 1, 1, 1, "1:01:01.000000001");
        assert_time(TimeBuilder::new().hours(1).negative().minutes(1).seconds(1).nanoseconds(1).build(),
                    -seconds - 1, -1, 1, 1, 1, 1, "-1:01:01.000000001");

        // +/- 00:00:59.999999999
        assert_time(TimeBuilder::new().seconds(59).nanoseconds(999999999).build(),
                    59, 1, 0, 0, 59, 999999999, "59.999999999s");
        assert_time(TimeBuilder::new().negative().seconds(59).nanoseconds(999999999).build(),
                    -60, -1, 0, 0, 59, 999999999, "-59.999999999s");

        // +/- 00:59:59.999999999
        let seconds = (60 * 60) - 1;
        assert_time(TimeBuilder::new().minutes(59).seconds(59).nanoseconds(999999999).build(),
                    seconds, 1, 0, 59, 59, 999999999, "59:59.999999999");
        assert_time(TimeBuilder::new().negative().minutes(59).seconds(59).nanoseconds(999999999).build(),
                    -seconds - 1, -1, 0, 59, 59, 999999999, "-59:59.999999999");

        // +/- 01:01:01.010101010
        let seconds = (60 * 60) + 60 + 1;
        assert_time(TimeBuilder::new().hours(1).minutes(1).seconds(1).nanoseconds(010101010).build(),
                    seconds, 1, 1, 1, 1, 010101010, "1:01:01.01010101");
        assert_time(TimeBuilder::new().negative().hours(1).minutes(1).seconds(1).nanoseconds(010101010).build(),
                    -seconds - 1, -1, 1, 1, 1, 010101010, "-1:01:01.01010101");
    }

    #[test]
    fn test_time_min_max() {
        assert_time(TimeBuilder::new().hours(2562047788015215).minutes(30).seconds(7).nanoseconds(999999999).build(),
                    std::i64::MAX, 1, 2562047788015215, 30, 7, 999999999, "2562047788015215:30:07.999999999");
        assert_time(TimeBuilder::new().negative().hours(2562047788015215).minutes(30).seconds(7).nanoseconds(999999999).build(),
                    std::i64::MIN, -1, 2562047788015215, 30, 7, 999999999, "-2562047788015215:30:07.999999999");
    }

    fn assert_time(time: Time, total_seconds: i64, signum: i64, hours: u64, minutes: u8, seconds: u8, nanoseconds: u32, time_string: &str) {
        assert_eq!(total_seconds, time.total_seconds());
        assert_eq!(signum, time.signum());
        assert_eq!(hours, time.hours());
        assert_eq!(minutes, time.minutes());
        assert_eq!(seconds, time.seconds());
        assert_eq!(nanoseconds, time.nanoseconds());
        assert_eq!(time_string, time.to_string());
    }
}
