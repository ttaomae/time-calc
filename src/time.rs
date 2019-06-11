use std::fmt;

use rust_decimal::Decimal;
use rust_decimal::RoundingStrategy;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal_macros::dec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

    /// Returns a new time builder.
    fn builder() -> TimeBuilder {
        TimeBuilder {
            negative: false,
            hours: 0,
            minutes: 0,
            seconds: 0,
            nanoseconds: 0,
        }
    }

    fn to_decimal(&self) -> Decimal {
        Decimal::new(self.seconds, 0) + Decimal::new(self.nanoseconds as i64, 9)
    }

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

// time + time
impl std::ops::Add for Time {
    type Output = Time;
    fn add(self, other: Time) -> Time {
        (self.to_decimal() + other.to_decimal()).to_time()
    }
}

// time - time
impl std::ops::Sub for Time {
    type Output = Time;
    fn sub(self, other: Time) -> Self::Output {
        (self.to_decimal() - other.to_decimal()).to_time()
    }
}

// time / time
impl std::ops::Div for Time {
    type Output = Decimal;
    fn div(self, other: Time) -> Decimal {
        self.to_decimal() / other.to_decimal()
    }
}

// time / decimal
impl std::ops::Div<Decimal> for Time {
    type Output = Time;
    fn div(self, other: Decimal) -> Time {
        (self.to_decimal() / other).to_time()
    }
}

// time * decimal
impl std::ops::Mul<Decimal> for Time {
    type Output = Time;
    fn mul(self, other: Decimal) -> Time {
        (self.to_decimal() * other).to_time()
    }
}

// decimal * time
impl std::ops::Mul<Time> for Decimal {
    type Output = Time;
    fn mul(self, other: Time) -> Time {
        other * self
    }
}

trait ToTime {
    fn to_time(&self) -> Time;
}

impl ToTime for Decimal {
    fn to_time(&self) -> Time {
        let seconds_per_hour = Decimal::new(Time::SECONDS_PER_HOUR as i64, 0);
        let seconds_per_minute = Decimal::new(Time::SECONDS_PER_MINUTE as i64, 0);

        let mut time_builder = Time::builder();
        if self.is_sign_negative() {
            time_builder.negative();
        }

        time_builder.hours((self / seconds_per_hour).abs().to_u64().unwrap());
        time_builder.minutes((self % seconds_per_hour / seconds_per_minute).abs().to_u8().unwrap());
        time_builder.seconds((self % seconds_per_minute).abs().to_u8().unwrap());

        // `self` may not have enough decimal places. Multiplying by 1.000000000 should ensure we
        // have at least nanosecond precision.
        let mut nanos = (self * dec!(1.000000000))
            // Round to 9 decimal places.
            .round_dp_with_strategy(9, RoundingStrategy::RoundHalfUp)
            // Keep only fractional part.
            .fract();
        // Convert fractional part to number of nanoseconds.
        nanos.set_scale(0);

        time_builder.nanoseconds(nanos.abs().to_u32().unwrap());
        time_builder.build()
    }
}

#[cfg(test)]
mod tests {
    use crate::time::Time;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    #[test]
    fn test_time_seconds_only() {
        // 00:00:00
        assert_time(Time::builder().build(),
                    0, 0, 0, 0, 0, 0, "0s");
        assert_time(Time::builder().negative().build(),
                    0, 0, 0, 0, 0, 0, "0s");

        // +/- 00:00:01
        assert_time(Time::builder().seconds(1).build(),
                    1, 1, 0, 0, 1, 0, "1s");
        assert_time(Time::builder().negative().seconds(1).build(),
                    -1, -1, 0, 0, 1, 0, "-1s");

        // +/- 00:01:00
        assert_time(Time::builder().minutes(1).build(),
                    60, 1, 0, 1, 0, 0, "01:00");
        assert_time(Time::builder().negative().minutes(1).build(),
                    -60, -1, 0, 1, 0, 0, "-01:00");

        // +/- 01:00:00
        let seconds = 60 * 60;
        assert_time(Time::builder().hours(1).build(),
                    seconds, 1, 1, 0, 0, 0, "1:00:00");
        assert_time(Time::builder().negative().hours(1).build(),
                    -seconds, -1, 1, 0, 0, 0, "-1:00:00");

        // +/- 00:01:01
        assert_time(Time::builder().minutes(1).seconds(1).build(),
                    61, 1, 0, 1, 1, 0, "01:01");
        assert_time(Time::builder().negative().minutes(1).seconds(1).build(),
                    -61, -1, 0, 1, 1, 0, "-01:01");

        // +/- 01:01:01
        let seconds = (60 * 60) + 60 + 1;
        assert_time(Time::builder().hours(1).minutes(1).seconds(1).build(),
                seconds, 1, 1, 1, 1, 0, "1:01:01");
        assert_time(Time::builder().negative().hours(1).minutes(1).seconds(1).build(),
                    -seconds, -1, 1, 1, 1, 0, "-1:01:01");

        // +/- 00:00:59
        assert_time(Time::builder().seconds(59).build(),
                    59, 1, 0, 0, 59, 0, "59s");
        assert_time(Time::builder().negative().seconds(59).build(),
                    -59, -1, 0, 0, 59, 0, "-59s");

        // +/- 00:59:59
        let seconds = (60 * 60) - 1;
        assert_time(Time::builder().minutes(59).seconds(59).build(),
                    seconds, 1, 0, 59, 59, 0, "59:59");
        assert_time(Time::builder().negative().minutes(59).seconds(59).build(),
                    -seconds, -1, 0, 59, 59, 0, "-59:59");
    }

    #[test]
    fn test_time_nanos_only() {
        // +/- 0.000000001s
        assert_time(Time::builder().nanoseconds(1).build(),
                    0, 1, 0, 0, 0, 1, "0.000000001s");
        assert_time(Time::builder().negative().nanoseconds(1).build(),
                    -1, -1, 0, 0, 0, 1, "-0.000000001s");

        // +/- 0.999999999s
        assert_time(Time::builder().nanoseconds(999999999).build(),
                    0, 1, 0, 0, 0, 999999999, "0.999999999s");
        assert_time(Time::builder().negative().nanoseconds(999999999).build(),
                    -1, -1, 0, 0, 0, 999999999, "-0.999999999s");

        // +/- 0.123456789
        assert_time(Time::builder().nanoseconds(123456789).build(),
                    0, 1, 0, 0 ,0, 123456789, "0.123456789s");
        assert_time(Time::builder().negative().nanoseconds(123456789).build(),
                    -1, -1, 0, 0, 0, 123456789, "-0.123456789s");

        // +/- 0.987654321
        assert_time(Time::builder().nanoseconds(987654321).build(),
                    0, 1, 0, 0, 0, 987654321, "0.987654321s");
        assert_time(Time::builder().negative().nanoseconds(987654321).build(),
                    -1, -1, 0, 0, 0, 987654321, "-0.987654321s");

        // +/- 0.13579
        assert_time(Time::builder().nanoseconds(135790000).build(),
                    0, 1, 0, 0, 0, 135790000, "0.13579s");
        assert_time(Time::builder().negative().nanoseconds(135790000).build(),
                    -1, -1, 0, 0, 0, 135790000, "-0.13579s");

        // +/- 0.2468
        assert_time(Time::builder().nanoseconds(246800000).build(),
                    0, 1, 0, 0, 0, 246800000, "0.2468s");
        assert_time(Time::builder().negative().nanoseconds(246800000).build(),
                    -1, -1, 0, 0, 0, 246800000, "-0.2468s");
    }

    #[test]
    fn test_time_seconds_and_nanos() {
        // +/- 00:00:01.000000001
        assert_time(Time::builder().seconds(1).nanoseconds(1).build(),
                    1, 1, 0, 0, 1, 1, "1.000000001s");
        assert_time(Time::builder().negative().seconds(1).nanoseconds(1).build(),
                    -2, -1, 0, 0, 1, 1, "-1.000000001s");

        // +/- 00:01:01.000000001
        assert_time(Time::builder().minutes(1).seconds(1).nanoseconds(1).build(),
                    61, 1, 0, 1, 1, 1, "01:01.000000001");
        assert_time(Time::builder().negative().minutes(1).seconds(1).nanoseconds(1).build(),
                    -62, -1, 0, 1, 1, 1, "-01:01.000000001");

        // +/- 01:01:01.000000001
        let seconds = (60 * 60) + 60 + 1;
        assert_time(Time::builder().hours(1).minutes(1).seconds(1).nanoseconds(1).build(),
                    seconds, 1, 1, 1, 1, 1, "1:01:01.000000001");
        assert_time(Time::builder().hours(1).negative().minutes(1).seconds(1).nanoseconds(1).build(),
                    -seconds - 1, -1, 1, 1, 1, 1, "-1:01:01.000000001");

        // +/- 00:00:59.999999999
        assert_time(Time::builder().seconds(59).nanoseconds(999999999).build(),
                    59, 1, 0, 0, 59, 999999999, "59.999999999s");
        assert_time(Time::builder().negative().seconds(59).nanoseconds(999999999).build(),
                    -60, -1, 0, 0, 59, 999999999, "-59.999999999s");

        // +/- 00:59:59.999999999
        let seconds = (60 * 60) - 1;
        assert_time(Time::builder().minutes(59).seconds(59).nanoseconds(999999999).build(),
                    seconds, 1, 0, 59, 59, 999999999, "59:59.999999999");
        assert_time(Time::builder().negative().minutes(59).seconds(59).nanoseconds(999999999).build(),
                    -seconds - 1, -1, 0, 59, 59, 999999999, "-59:59.999999999");

        // +/- 01:01:01.010101010
        let seconds = (60 * 60) + 60 + 1;
        assert_time(Time::builder().hours(1).minutes(1).seconds(1).nanoseconds(010101010).build(),
                    seconds, 1, 1, 1, 1, 010101010, "1:01:01.01010101");
        assert_time(Time::builder().negative().hours(1).minutes(1).seconds(1).nanoseconds(010101010).build(),
                    -seconds - 1, -1, 1, 1, 1, 010101010, "-1:01:01.01010101");
    }

    #[test]
    fn test_time_min_max() {
        assert_time(Time::builder().hours(2562047788015215).minutes(30).seconds(7).nanoseconds(999999999).build(),
                    std::i64::MAX, 1, 2562047788015215, 30, 7, 999999999, "2562047788015215:30:07.999999999");
        assert_time(Time::builder().negative().hours(2562047788015215).minutes(30).seconds(7).nanoseconds(999999999).build(),
                    std::i64::MIN, -1, 2562047788015215, 30, 7, 999999999, "-2562047788015215:30:07.999999999");
    }

    #[test]
    fn test_add_sub_positive() {
        let zero = time(0, 0, 0, 0);

        // Nanoseconds.
        let _1ns = time(0, 0, 0, 1);
        let _200000000ns = time(0, 0, 0, 200000000);
        let _500000000ns = time(0, 0, 0, 500000000);
        let _800000000ns = time(0, 0, 0, 800000000);
        let _999999999ns = time(0, 0, 0, 999999999);

        // Seconds.
        let _1s = time(0, 0, 1, 0);
        let _2s = time(0, 0, 2, 0);
        let _3s = time(0, 0, 3, 0);
        let _10s = time(0, 0, 10, 0);
        let _12s = time(0, 0, 12,0);
        let _34s = time(0, 0, 34,0);
        let _59s = time(0, 0, 59, 0);

        // Minutes.
        let _1m = time(0, 1, 0, 0);
        let _2m = time(0, 2, 0, 0);
        let _3m = time(0, 3, 0, 0);
        let _15m = time(0, 15, 0, 0);
        let _37m = time(0, 37, 0, 0);
        let _59m = time(0, 59, 0, 0);

        // Hours.
        let _1h = time(1, 0, 0, 0);
        let _2h = time(2, 0, 0, 0);
        let _3h = time(3, 0, 0, 0);
        let _7h = time(7, 0, 0, 0);

        assert_eq!(zero + zero, zero);
        assert_eq!(zero - zero, zero);

        // Add nanoseconds.
        assert_eq!(zero + _1ns, _1ns);
        assert_eq!(_1ns + zero, _1ns);
        assert_eq!(_1ns + _999999999ns, _1s);
        assert_eq!(_200000000ns + _800000000ns, _1s);
        assert_eq!(_500000000ns + _800000000ns, time(0, 0, 1, 300000000));

        // Add seconds.
        assert_eq!(zero + _1s, _1s);
        assert_eq!(_1s + zero, _1s);
        assert_eq!(_1s + _1s, _2s);
        assert_eq!(_1s + _2s, _3s);
        assert_eq!(_2s + _1s, _3s);
        assert_eq!(_1s + _1s + _1s, _3s);
        assert_eq!(_1s + _1s + _2s + _3s + _3s, _10s);
        assert_eq!(_59s + _1s, _1m);
        assert_eq!(_12s + _34s, time(0, 0, 46, 0));
        assert_eq!(_34s + _34s, time(0, 1, 8, 0));

        // Add minutes.
        assert_eq!(zero + _1m, _1m);
        assert_eq!(_1m + zero, _1m);
        assert_eq!(_1m + _1m, _2m);
        assert_eq!(_1m + _2m, _3m);
        assert_eq!(_2m + _1m, _3m);
        assert_eq!(_1m + _1m + _1m, _3m);
        assert_eq!(_59m + _1m, _1h);
        assert_eq!(_15m + _37m, time(0, 52, 0, 0));
        assert_eq!(_37m + _37m, time(1, 14, 0, 0));

        // Add hours.
        assert_eq!(zero + _1h, _1h);
        assert_eq!(_1h + zero, _1h);
        assert_eq!(_1h + _2h, _3h);
        assert_eq!(_2h + _1h, _3h);
        assert_eq!(_1h + _1h + _1h, _3h);

        // Add seconds and nanos.
        assert_eq!(_59s + _999999999ns + _1ns, _1m);
        assert_eq!(_1s + _200000000ns, time(0, 0, 1, 200000000));
        assert_eq!(_12s + _800000000ns + _500000000ns, time(0, 0, 13, 300000000));
        assert_eq!(_34s + _34s + _800000000ns + _500000000ns, time(0, 1, 9, 300000000));

        // Add minutes and seconds.
        assert_eq!(_59m + _59s + _1s, _1h);
        assert_eq!(_1m + _1s, time(0, 1, 1, 0));
        assert_eq!(_15m + _34s, time(0, 15, 34, 0));
        assert_eq!(_37m + _12s, time(0, 37, 12, 0));
        assert_eq!(_37m + _37m + _34s + _34s, time(1, 15, 8, 0));

        // Add hours and minutes.
        assert_eq!(_1h + _1m, time(1, 1, 0, 0));
        assert_eq!(_2h + _37m + _37m, time(3, 14, 0, 0));
        assert_eq!(_1h + _7h + _37m + _15m + _15m, time(9, 7, 0, 0));

        // Add all.
        assert_eq!(_1h + _1m + _1s + _1ns, time(1, 1, 1, 1));
        assert_eq!(_59m + _59s + _999999999ns + _1ns, _1h);
        assert_eq!(_3h + _37m + _37m + _34s + _34s + _800000000ns + _800000000ns, time(4, 15, 9, 600000000));

        // Sub nanoseconds.
        assert_eq!(_1ns - _1ns, zero);
        assert_eq!(_1ns - zero, _1ns);
        assert_eq!(zero - _1ns, neg_time(0, 0, 0, 1));
        assert_eq!(_800000000ns - _500000000ns, time(0, 0, 0, 300000000));
        assert_eq!(_200000000ns - _800000000ns, neg_time(0, 0, 0, 600000000));

        // Sub seconds.
        assert_eq!(_1s - _1s, zero);
        assert_eq!(_1s - zero, _1s);
        assert_eq!(zero - _1s, neg_time(0, 0, 1, 0));
        assert_eq!(_34s - _12s, time(0, 0, 22, 0));
        assert_eq!(_12s - _34s, neg_time(0, 0, 22, 0));

        // sub minutes.
        assert_eq!(_1m - _1m, zero);
        assert_eq!(_1m - zero, _1m);
        assert_eq!(zero - _1m, neg_time(0, 1, 0, 0));
        assert_eq!(_37m - _15m, time(0, 22, 0, 0));
        assert_eq!(_15m - _37m, neg_time(0, 22, 0, 0));

        // Sub hours.
        assert_eq!(_1h - _1h, zero);
        assert_eq!(_1h - zero, _1h);
        assert_eq!(zero - _1h, neg_time(1, 0, 0, 0));
        assert_eq!(_7h - _3h, time(4, 0, 0, 0));
        assert_eq!(_3h - _7h, neg_time(4, 0, 0, 0));

        // Sub all.
        assert_eq!(zero - _1h - _1m - _1s - _1ns, neg_time(1, 1, 1, 1));
        assert_eq!(_1h - _1ns, time(0, 59, 59, 999999999));
        assert_eq!(_1m - _1ns, time(0, 0, 59, 999999999));
        assert_eq!(_1s - _1ns, time(0, 0, 0, 999999999));
        assert_eq!(_1h - _15m - _34s - _200000000ns, time(0, 44, 25, 800000000));
    }

    #[test]
    fn test_add_sub_negative() {
        let zero = time(0, 0, 0, 0);

        // Nanoseconds.
        let _1ns = time(0, 0, 0, 1);
        let neg_1ns = neg_time(0, 0, 0, 1);

        // Seconds.
        let _1s = time(0, 0, 1, 0);
        let neg_1s = neg_time(0, 0, 1, 0);

        // Minutes.
        let _1m = time(0, 1, 0, 0);
        let neg_1m = neg_time(0, 1, 0, 0);

        // Hours.
        let _1h = time(1, 0, 0, 0);
        let neg_1h = neg_time(1, 0, 0, 0);

        // Add nanoseconds.
        assert_eq!(_1ns + neg_1ns, zero);
        assert_eq!(neg_1ns + _1ns, zero);
        assert_eq!(neg_1ns + neg_1ns, neg_time(0, 0, 0, 2));

        // Add seconds.
        assert_eq!(_1s + neg_1s, zero);
        assert_eq!(neg_1s + _1s, zero);
        assert_eq!(neg_1s + neg_1s, neg_time(0, 0, 2, 0));

        // Add minutes.
        assert_eq!(_1m + neg_1m, zero);
        assert_eq!(neg_1m + _1m, zero);
        assert_eq!(neg_1m + neg_1m, neg_time(0, 2, 0, 0));

        // Add hours.
        assert_eq!(_1h + neg_1h, zero);
        assert_eq!(neg_1h + _1h, zero);
        assert_eq!(neg_1h + neg_1h, neg_time(2, 0, 0, 0));

        // Sub nanoseconds.
        assert_eq!(_1ns - neg_1ns, time(0, 0, 0, 2));
        assert_eq!(neg_1ns - _1ns, neg_time(0, 0, 0, 2));
        assert_eq!(neg_1ns - neg_1ns, zero);

        // Sub seconds.
        assert_eq!(_1s - neg_1s, time(0, 0, 2, 0));
        assert_eq!(neg_1s - _1s, neg_time(0, 0, 2, 0));
        assert_eq!(neg_1s - neg_1s, zero);

        // Sub minutes.
        assert_eq!(_1m - neg_1m, time(0, 2, 0, 0));
        assert_eq!(neg_1m - _1m, neg_time(0, 2, 0, 0));
        assert_eq!(neg_1m - neg_1m, zero);

        // Sub hours.
        assert_eq!(_1h - neg_1h, time(2, 0, 0, 0));
        assert_eq!(neg_1h - _1h, neg_time(2, 0, 0, 0));
        assert_eq!(neg_1h - neg_1h, zero);

        // Add/Sub all.
        assert_eq!(neg_1h + neg_1m + neg_1s + neg_1ns, neg_time(1, 1, 1, 1));
        assert_eq!(_1h + neg_1ns, time(0, 59, 59, 999999999));
        assert_eq!(_1m + neg_1ns, time(0, 0, 59, 999999999));
        assert_eq!(_1s + neg_1ns, time(0, 0, 0, 999999999));
        assert_eq!(neg_1h + _1ns, neg_time(0, 59, 59, 999999999));
        assert_eq!(neg_1m + _1ns, neg_time(0, 0, 59, 999999999));
        assert_eq!(neg_1s + _1ns, neg_time(0, 0, 0, 999999999));
    }

    #[test]
    fn test_div_time() {
        let zero = time(0, 0, 0, 0);

        // Nanoseconds.
        let _1ns = time(0, 0, 0, 1);
        let neg_1ns = neg_time(0, 0, 0, 1);

        // Seconds.
        let _1s = time(0, 0, 1, 0);
        let neg_1s = neg_time(0, 0, 1, 0);

        // Minutes.
        let _1m = time(0, 1, 0, 0);
        let neg_1m = neg_time(0, 1, 0, 0);

        // Hours.
        let _1h = time(1, 0, 0, 0);
        let neg_1h = neg_time(1, 0, 0, 0);

        // All.
        let _1h_1m_1s_1ns = time(1, 1, 1, 1);
        let neg_1h_1m_1s_1ns = neg_time(1, 1, 1, 1);

        // 0 / x
        assert_eq!(zero / _1ns, dec!(0));
        assert_eq!(zero / _1s, dec!(0));
        assert_eq!(zero / _1m, dec!(0));
        assert_eq!(zero / _1h, dec!(0));
        assert_eq!(zero / _1h_1m_1s_1ns, dec!(0));
        assert_eq!(zero / neg_1ns, dec!(0));
        assert_eq!(zero / neg_1s, dec!(0));
        assert_eq!(zero / neg_1m, dec!(0));
        assert_eq!(zero / neg_1h, dec!(0));
        assert_eq!(zero / neg_1h_1m_1s_1ns, dec!(0));

        // Div nanoseconds.
        assert_eq!(_1ns / _1ns, dec!(1));
        assert_eq!(_1ns / neg_1ns, dec!(-1));
        assert_eq!(neg_1ns / _1ns, dec!(-1));
        assert_eq!(neg_1ns / neg_1ns, dec!(1));

        // Div seconds.
        assert_eq!(_1s / _1s, dec!(1));
        assert_eq!(_1s / neg_1s, dec!(-1));
        assert_eq!(neg_1s / _1s, dec!(-1));
        assert_eq!(neg_1s / neg_1s, dec!(1));

        assert_eq!(_1s / _1ns, dec!(1_000_000_000));
        assert_eq!(_1s / time(0, 0, 0, 500000000), dec!(2.0));

        // Div minutes.
        assert_eq!(_1m / _1m, dec!(1));
        assert_eq!(_1m / neg_1m, dec!(-1));
        assert_eq!(neg_1m / _1m, dec!(-1));
        assert_eq!(neg_1m / neg_1m, dec!(1));

        assert_eq!(_1m / _1s, dec!(60));
        assert_eq!(_1m / _1ns, dec!(60_000_000_000));

        assert_eq!(_1m / time(0, 0, 30, 0), dec!(2.0));
        assert_eq!(_1m / time(0, 0, 15, 0), dec!(4.0));
        assert_eq!(_1m / time(0, 0, 10, 0), dec!(6.0));
        assert_eq!(_1m / time(0, 0, 5, 0), dec!(12.0));
        assert_eq!(_1m / time(0, 0, 3, 0), dec!(20.0));

        assert_eq!(_1m / time(0, 0, 2, 500000000), dec!(24.0));
        assert_eq!(_1m / time(0, 0, 16, 0), dec!(3.75));
        assert_eq!(_1m / time(0, 4, 0, 0), dec!(0.25));
        assert_eq!(_1m / time(0, 2, 30, 0), dec!(0.4));

        // Div hours.
        assert_eq!(_1h / _1h, dec!(1));
        assert_eq!(_1h / neg_1h, dec!(-1));
        assert_eq!(neg_1h / _1h, dec!(-1));
        assert_eq!(neg_1h / neg_1h, dec!(1));

        assert_eq!(_1h / _1m, dec!(60));
        assert_eq!(_1h / _1s, dec!(3600));
        assert_eq!(_1h / _1ns, dec!(3_600_000_000_000));

        assert_eq!(_1h / time(0, 20, 0, 0), dec!(3.0));
        assert_eq!(_1h / time(0, 12, 0, 0), dec!(5.0));
        assert_eq!(_1h / time(0, 6, 0, 0), dec!(10.0));
        assert_eq!(_1h / time(0, 4, 0, 0), dec!(15.0));
        assert_eq!(_1h / time(0, 2, 0, 0), dec!(30.0));

        assert_eq!(_1h / time(0, 7, 30, 0), dec!(8));
        assert_eq!(_1h / time(0, 24, 0, 0), dec!(2.5));
        assert_eq!(_1h / time(2, 30, 0, 0), dec!(0.4));

        // Div all.
        assert_eq!(_1h_1m_1s_1ns / _1h_1m_1s_1ns, dec!(1));
        assert_eq!(_1h_1m_1s_1ns / neg_1h_1m_1s_1ns, dec!(-1));
        assert_eq!(neg_1h_1m_1s_1ns / _1h_1m_1s_1ns, dec!(-1));
        assert_eq!(neg_1h_1m_1s_1ns / neg_1h_1m_1s_1ns, dec!(1));

        assert_eq!(_1h_1m_1s_1ns / _1ns, dec!(3661000000001));
        assert_eq!(_1h_1m_1s_1ns / _1s, dec!(3661.000000001));
        //          1h:1m        /  1m    == 61
        //                1s     /  1m    == 00.0166666666666666...
        //                   1ns /  1m    == 00.0000000000166666...
        assert_eq!(_1h_1m_1s_1ns / _1m, dec!(61.0166666666833333333333333333));
        //             1m        /  1h    == 0.01666666666666666...
        //                1s     /  1h    == 0.00027777777777777...
        //                   1ns /  1h    == 0.00000000000027777...
        assert_eq!(_1h_1m_1s_1ns / _1h, dec!(1.01694444444472222222222222222));
    }

    #[test]
    fn test_div_num() {
        let zero = time(0, 0, 0, 0);
        assert_eq!(zero / dec!(1.0), zero);
        assert_eq!(zero / dec!(1.5), zero);
        assert_eq!(zero / dec!(2.0), zero);
        assert_eq!(zero / dec!(12345.0), zero);

        // Div nanoseconds.
        let _1ns = time(0, 0, 0, 1);
        let neg_1ns = neg_time(0, 0, 0, 1);
        assert_eq!(_1ns / dec!(1), _1ns);
        assert_eq!(_1ns / dec!(1.00), _1ns);
        assert_eq!(_1ns / dec!(-1.0), neg_1ns);
        assert_eq!(neg_1ns / dec!(1.0), neg_1ns);
        assert_eq!(neg_1ns / dec!(-1.0), _1ns);

        assert_eq!(_1ns / dec!(2.0), _1ns); // 0.5ns rounds up to 1ns.
        assert_eq!(_1ns / dec!(3.0), zero); // 0.333ns rounds down to 0ns.

        assert_eq!(_1ns / dec!(0.5), time(0, 0, 0, 2));
        assert_eq!(_1ns / dec!(0.1), time(0, 0, 0, 10));

        // Div seconds.
        let _1s = time(0, 0, 1, 0);
        let _2s = time(0, 0, 2, 0);
        let neg_1s = neg_time(0, 0, 1, 0);
        assert_eq!(_1s / dec!(1), _1s);
        assert_eq!(_1s / dec!(1.00), _1s);
        assert_eq!(_1s / dec!(-1.0), neg_1s);
        assert_eq!(neg_1s / dec!(1.0), neg_1s);
        assert_eq!(neg_1s / dec!(-1.0), _1s);

        assert_eq!(_1s / dec!(2.0), time(0, 0, 0, 500000000));
        assert_eq!(_1s / dec!(2.000), time(0, 0, 0, 500000000));
        assert_eq!(_1s / dec!(0.5), time(0, 0, 2, 0));
        assert_eq!(_1s / dec!(0.500), time(0, 0, 2, 0));

        assert_eq!(_1s / dec!(3.0), time(0, 0, 0, 333333333));
        assert_eq!(_1s / dec!(6.0), time(0, 0, 0, 166666667));
        assert_eq!(_1s / dec!(9.0), time(0, 0, 0, 111111111));
        assert_eq!(_2s / dec!(3.0), time(0, 0, 0, 666666667));
        assert_eq!(_2s / dec!(6.0), time(0, 0, 0, 333333333));
        assert_eq!(_2s / dec!(9.0), time(0, 0, 0, 222222222));
        assert_eq!(_1s / dec!(4.2), time(0, 0, 0, 238095238));

        // Div minutes.
        let _1m = time(0, 1, 0, 0);
        let neg_1m = neg_time(0, 1, 0, 0);
        assert_eq!(_1m / dec!(1), _1m);
        assert_eq!(_1m / dec!(1.00), _1m);
        assert_eq!(_1m / dec!(-1.0), neg_1m);
        assert_eq!(neg_1m / dec!(1.0), neg_1m);
        assert_eq!(neg_1m / dec!(-1.0), _1m);

        assert_eq!(_1m / dec!(2.0), time(0, 0, 30, 0));
        assert_eq!(_1m / dec!(4.0), time(0, 0, 15, 0));
        assert_eq!(_1m / dec!(6.0), time(0, 0, 10, 0));
        assert_eq!(_1m / dec!(12.0), time(0, 0, 5, 0));
        assert_eq!(_1m / dec!(20.0), time(0, 0, 3, 0));

        assert_eq!(_1m / dec!(7.0), time(0, 0, 8, 571428571));
        assert_eq!(_1m / dec!(8.0), time(0, 0, 7, 500000000));
        assert_eq!(_1m / dec!(9.0), time(0, 0, 6, 666666667));
        assert_eq!(_1m / dec!(4.2), time(0, 0, 14, 285714286));

        // Div hours.
        let _1h = time(1, 0, 0, 0);
        let neg_1h = neg_time(1, 0, 0, 0);
        assert_eq!(_1h / dec!(1), _1h);
        assert_eq!(_1h / dec!(1.00), _1h);
        assert_eq!(_1h / dec!(-1.0), neg_1h);
        assert_eq!(neg_1h / dec!(1.0), neg_1h);
        assert_eq!(neg_1h / dec!(-1.0), _1h);

        assert_eq!(_1h / dec!(3.0), time(0, 20, 0, 0));
        assert_eq!(_1h / dec!(5.0), time(0, 12, 0, 0));
        assert_eq!(_1h / dec!(10.0), time(0, 6, 0, 0));
        assert_eq!(_1h / dec!(15.0), time(0, 4, 0, 0));
        assert_eq!(_1h / dec!(30.0), time(0, 2, 0, 0));

        assert_eq!(_1h / dec!(7.0), time(0, 8, 34, 285714286));
        assert_eq!(_1h / dec!(8.0), time(0, 7, 30, 0));
        assert_eq!(_1h / dec!(9.0), time(0, 6, 40, 0));
        assert_eq!(_1h / dec!(4.2), time(0, 14, 17, 142857143));

        // Div all.
        let _1h_1m_1s_1ns = time(1, 1, 1, 1);
        let neg_1h_1m_1s_1ns = neg_time(1, 1, 1, 1);
        assert_eq!(_1h_1m_1s_1ns / dec!(1), _1h_1m_1s_1ns);
        assert_eq!(_1h_1m_1s_1ns / dec!(1), _1h_1m_1s_1ns);
        assert_eq!(_1h_1m_1s_1ns / dec!(1.00), _1h_1m_1s_1ns);
        assert_eq!(_1h_1m_1s_1ns / dec!(-1.0), neg_1h_1m_1s_1ns);
        assert_eq!(neg_1h_1m_1s_1ns / dec!(1.0), neg_1h_1m_1s_1ns);
        assert_eq!(neg_1h_1m_1s_1ns / dec!(-1.0), _1h_1m_1s_1ns);

        assert_eq!(_1h_1m_1s_1ns / dec!(2.0), time(0, 30, 30, 500000001));
        assert_eq!(_1h_1m_1s_1ns / dec!(3.0), time(0, 20, 20, 333333334));
        assert_eq!(_1h_1m_1s_1ns / dec!(4.0), time(0, 15, 15, 250000000));
        assert_eq!(_1h_1m_1s_1ns / dec!(4.2), time(0, 14, 31, 666666667));
    }

    #[test]
    fn test_mul() {
        let zero = time(0, 0, 0, 0);

        // Nanoseconds.
        let _1ns = time(0, 0, 0, 1);
        let neg_1ns = neg_time(0, 0, 0, 1);

        // Seconds.
        let _1s = time(0, 0, 1, 0);
        let neg_1s = neg_time(0, 0, 1, 0);

        // Minutes.
        let _1m = time(0, 1, 0, 0);
        let neg_1m = neg_time(0, 1, 0, 0);

        // Hours.
        let _1h = time(1, 0, 0, 0);
        let neg_1h = neg_time(1, 0, 0, 0);

        // All.
        let _1h_1m_1s_1ns = time(1, 1, 1, 1);
        let neg_1h_1m_1s_1ns = neg_time(1, 1, 1, 1);

        // x * 0.0
        assert_mul(zero, dec!(0.0), zero);
        assert_mul(_1ns, dec!(0.0), zero);
        assert_mul(_1s, dec!(0.0), zero);
        assert_mul(_1m, dec!(0.0), zero);
        assert_mul(_1h, dec!(0.0), zero);
        assert_mul(_1h_1m_1s_1ns, dec!(0.0), zero);

        // x * 1.0
        assert_mul(_1ns, dec!(1.0), _1ns);
        assert_mul(_1s, dec!(1.0), _1s);
        assert_mul(_1m, dec!(1.0), _1m);
        assert_mul(_1h, dec!(1.0), _1h);
        assert_mul(_1h_1m_1s_1ns, dec!(1.0), _1h_1m_1s_1ns);
        assert_mul(neg_1ns, dec!(1.0), neg_1ns);
        assert_mul(neg_1s, dec!(1.0), neg_1s);
        assert_mul(neg_1m, dec!(1.0), neg_1m);
        assert_mul(neg_1h, dec!(1.0), neg_1h);
        assert_mul(neg_1h_1m_1s_1ns, dec!(1.0), neg_1h_1m_1s_1ns);

        // x * -1.0
        assert_mul(_1ns, dec!(-1.0), neg_1ns);
        assert_mul(_1s, dec!(-1.0), neg_1s);
        assert_mul(_1m, dec!(-1.0), neg_1m);
        assert_mul(_1h, dec!(-1.0), neg_1h);
        assert_mul(_1h_1m_1s_1ns, dec!(-1.0), neg_1h_1m_1s_1ns);
        assert_mul(neg_1ns, dec!(-1.0), _1ns);
        assert_mul(neg_1s, dec!(-1.0), _1s);
        assert_mul(neg_1m, dec!(-1.0), _1m);
        assert_mul(neg_1h, dec!(-1.0), _1h);
        assert_mul(neg_1h_1m_1s_1ns, dec!(-1.0), _1h_1m_1s_1ns);

        // Mul nanoseconds.
        assert_mul(_1ns, dec!(1234.0), time(0, 0, 0, 1234));
        assert_mul(_1ns, dec!(56789.0), time(0, 0, 0, 56789));

        // Mul seconds.
        assert_mul(_1s, dec!(0.25), time(0, 0, 0, 250000000));
        assert_mul(_1s, dec!(4.0), time(0, 0, 4, 0));
        assert_mul(_1s, dec!(75.0), time(0, 1, 15, 0));
        assert_mul(_1s, dec!(3661.0), time(1, 1, 1, 0));

        // Mul minutes.
        assert_mul(_1m, dec!(0.5), time(0, 0, 30, 0));
        assert_mul(_1m, dec!(12.2), time(0, 12, 12, 0));
        assert_mul(_1m, dec!(125.125), time(2, 5, 7, 500000000));

        // Mul hours.
        assert_mul(_1h, dec!(0.75), time(0, 45, 0, 0));
        assert_mul(_1h, dec!(13.579), time(13, 34, 44, 400000000));
        assert_mul(_1h, dec!(22.22), time(22, 13, 12, 0));

        // Mul all.
        assert_mul(_1h_1m_1s_1ns, dec!(3.0), time(3, 3, 3, 3));
        assert_mul(_1h_1m_1s_1ns, dec!(77), time(78, 18, 17, 77));
        // 1h * 8.8  = 8h 48m
        // 1m * 8.8  =     8m 48s
        // 1s * 8.8  =         8s 800000000ns
        // 1ns * 8.8 =                    9ns
        assert_mul(_1h_1m_1s_1ns, dec!(8.8), time(8, 56, 56, 800000009));
    }

    fn time(hours: u64, minutes: u8, seconds: u8, nanoseconds: u32) -> Time {
        Time::builder().hours(hours).minutes(minutes).seconds(seconds).nanoseconds(nanoseconds).build()
    }

    fn neg_time(hours: u64, minutes: u8, seconds: u8, nanoseconds: u32) -> Time {
        Time::builder().negative().hours(hours).minutes(minutes).seconds(seconds).nanoseconds(nanoseconds).build()
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

    /// Assert time * num == expected and num * time == expected.
    fn assert_mul(time: Time, num: Decimal, expected: Time) {
        assert_eq!(time * num, expected);
        assert_eq!(num * time, expected);
    }
}
