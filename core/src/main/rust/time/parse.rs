use std::fmt;
use std::iter::Peekable;
use std::result::Result;
use std::str::Chars;

use crate::time::Time;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Token {
    Hyphen,
    Number(String),
    Colon,
    FullStop,
    S,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let t = match self {
            Token::Hyphen => String::from("-"),
            Token::Number(n) => n.clone(),
            Token::Colon => String::from(":"),
            Token::FullStop => String::from("."),
            Token::S => String::from("s"),
        };

        write!(f, "{}", t)
    }
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    scan_complete: bool,
}

#[derive(Debug)]
pub enum LexError {
    UnexpectedCharacter(char),
    EndOfInput,
}

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
            tokens: Vec::new(),
            scan_complete: false,
        }
    }

    fn scan(&mut self) -> Result<Vec<Token>, Vec<LexError>> {
        let mut errors = Vec::new();

        if !self.scan_complete {
            while let Some(ch) = self.peek() {
                if ch.is_numeric() {
                    if let Result::Err(err) = self.scan_number() {
                        errors.push(err);
                    }
                } else if let Result::Err(err) = self.scan_character() {
                    errors.push(err);
                }
            }
        }

        self.scan_complete = true;
        if errors.is_empty() {
            Result::Ok(self.tokens.clone())
        } else {
            Result::Err(errors)
        }
    }

    fn scan_number(&mut self) -> Result<(), LexError> {
        let mut num = String::new();
        loop {
            match self.peek() {
                Option::Some(c) if c.is_digit(10) => {
                    num.push(*c);
                    self.next();
                }
                _ => break,
            }
        }

        self.tokens.push(Token::Number(num));
        Ok(())
    }

    fn scan_character(&mut self) -> Result<(), LexError> {
        let token = match self.next() {
            Option::Some('-') => Token::Hyphen,
            Option::Some(':') => Token::Colon,
            Option::Some('.') => Token::FullStop,
            Option::Some('s') => Token::S,
            Option::Some(c) => return Result::Err(LexError::UnexpectedCharacter(c)),
            Option::None => return Result::Err(LexError::EndOfInput),
        };

        self.tokens.push(token);
        Ok(())
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }
}

struct Parser {
    tokens: Vec<Token>,
}

#[derive(Debug)]
pub enum ParseError {
    LexError(Vec<LexError>),
    ExpectedNumber(Option<Token>),
    ExceededMaxComponents,
    ExpectedNumberAfterDecimal(Option<Token>),
    ExpectedSecondsIdentifier,
    UnexpectedSecondsIdentifier,
    ExpectedTwoDigitMinutes(String),
    ExpectedTwoDigitSeconds(String),
    SecondsOutOfRange(u8),
    MinutesOutOfRange(u8),
    FractionalSecondsTooLarge(String),
    ExpectedEndOfInputOrFraction(Option<Token>),
    ExpectedEndOfInput(Token),
}

impl std::convert::From<Vec<LexError>> for ParseError {
    fn from(lex_error: Vec<LexError>) -> Self {
        ParseError::LexError(lex_error)
    }
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    fn parse(self) -> Result<Time, ParseError> {
        let mut token_iter = self.tokens.into_iter().peekable();

        // Only consume first token if it is a hyphen, indicating a negative time.
        let is_negative = if token_iter.peek() == Option::Some(&Token::Hyphen) {
            token_iter.next();
            true
        }
        else {
            false
        };

        // Read numbers, separated by colons.
        let mut components = Vec::new();
        loop {
            match token_iter.next() {
                Option::Some(Token::Number(n)) => components.push(n),
                t => return Result::Err(ParseError::ExpectedNumber(t)),
            }
            match token_iter.peek() {
                Option::Some(Token::Colon) => {token_iter.next();},
                _ => break,
            }
        }
        // If we reach this point, we must have consumed at least one number from tokens so we don't
        // need to check for `components.len() == 0`.
        if components.len() > 3 {
            return Result::Err(ParseError::ExceededMaxComponents)
        }

        // Consume fractional seconds.
        let fraction = if let Option::Some(&Token::FullStop) = token_iter.peek() {
            // Consume full stop.
            token_iter.next();

            // Consume number.
            match token_iter.next() {
                Option::Some(Token::Number(n)) => Option::Some(n.to_string()),
                t => return Result::Err(ParseError::ExpectedNumberAfterDecimal(t)),
            }
        }
        else {
            Option::None
        };

        // Consume 's'.
        let is_seconds = if token_iter.peek() == Option::Some(&Token::S) {
            token_iter.next();
            true
        }
        else {
            false
        };

        // We've consumed everything we understand.
        if let Option::Some(t) = token_iter.peek() {
            return Result::Err(ParseError::ExpectedEndOfInput(t.clone()))
        }

        if is_seconds && components.len() != 1 {
            return Result::Err(ParseError::UnexpectedSecondsIdentifier);
        }
        if !is_seconds && components.len() == 1 {
            return Result::Err(ParseError::ExpectedSecondsIdentifier);
        }

        // Construct time.
        let mut time_builder = Time::builder();

        if is_negative {
            time_builder.negative();
        }

        // Hours.
        if components.len() >= 3 {
            let h = components[0].to_string();
            time_builder.hours(h.parse().unwrap());
        }
        // Minutes
        if components.len() >= 2 {
            let m = components[
                if components.len() == 2 { 0 }
                else { 1 }
            ].to_string();

            if m.len() != 2 {
                return Result::Err(ParseError::ExpectedTwoDigitMinutes(m));
            }
            let minutes = m.parse().unwrap();
            if minutes >= 60 {
                return Result::Err(ParseError::MinutesOutOfRange(minutes));
            }
            time_builder.minutes(minutes);
        }
        // Seconds
        if components.len() >= 1 {
            let s = components[
                if components.len() == 1 { 0 }
                else if components.len() == 2 { 1 }
                else { 2 }
            ].to_string();

            if s.len() != 2 && !is_seconds {
                return Result::Err(ParseError::ExpectedTwoDigitSeconds(s));
            }
            let seconds = s.parse().unwrap();
            if seconds >= 60 {
                return Result::Err(ParseError::SecondsOutOfRange(seconds));
            }
            time_builder.seconds(seconds);
        }
        // Nanoseconds
        if let Option::Some(mut ns) = fraction {
            if ns.len() > 9 {
                return Result::Err(ParseError::FractionalSecondsTooLarge(ns.to_string()));
            }
            while ns.len() < 9 {
                ns.push('0');
            }
            time_builder.nanoseconds(ns.parse().unwrap());
        }

        Result::Ok(time_builder.build())
    }
}

pub(crate) fn parse_time(time: &str) -> Result<Time, ParseError> {
    Parser::new(Lexer::new(time).scan()?).parse()
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::parse_time;
    use super::Lexer;
    use super::Token;
    use super::Token::*;
    use crate::time::Time;

    #[test]
    fn scan_single_token() {
        assert_scan_tokens("-", vec![Hyphen]);
        assert_scan_tokens(":", vec![Colon]);
        assert_scan_tokens(".", vec![FullStop]);
        assert_scan_tokens("0", vec![Number(0.to_string())]);
        assert_scan_tokens("1", vec![Number(1.to_string())]);
        assert_scan_tokens("2", vec![Number(2.to_string())]);
        assert_scan_tokens("3", vec![Number(3.to_string())]);
        assert_scan_tokens("4", vec![Number(4.to_string())]);
        assert_scan_tokens("5", vec![Number(5.to_string())]);
        assert_scan_tokens("6", vec![Number(6.to_string())]);
        assert_scan_tokens("7", vec![Number(7.to_string())]);
        assert_scan_tokens("8", vec![Number(8.to_string())]);
        assert_scan_tokens("9", vec![Number(9.to_string())]);
        assert_scan_tokens("00", vec![Number("00".to_string())]);
        assert_scan_tokens("59", vec![Number(59.to_string())]);
        assert_scan_tokens("13579", vec![Number(13579.to_string())]);
        assert_scan_tokens("24680", vec![Number(24680.to_string())]);
        assert_scan_tokens("123456789", vec![Number(123456789.to_string())]);
    }

    #[test]
    fn scan_multiple_tokens() {
        assert_scan_tokens("--", vec![Hyphen, Hyphen]);
        assert_scan_tokens("::", vec![Colon, Colon]);
        assert_scan_tokens("..", vec![FullStop, FullStop]);
        assert_scan_tokens("-:", vec![Hyphen, Colon]);
        assert_scan_tokens(".-", vec![FullStop, Hyphen]);
        assert_scan_tokens(
            "...---...",
            vec![
                FullStop, FullStop, FullStop,
                Hyphen, Hyphen, Hyphen,
                FullStop, FullStop, FullStop,
            ],
        );
        assert_scan_tokens(
            ":-.:-.:.-:.-:",
            vec![
                Colon, Hyphen, FullStop,
                Colon, Hyphen, FullStop,
                Colon, FullStop, Hyphen,
                Colon, FullStop, Hyphen,
                Colon,
            ],
        );

        assert_scan_tokens("-38", vec![Hyphen, Number(38.to_string())]);
        assert_scan_tokens("62.", vec![Number(62.to_string()), FullStop]);
        assert_scan_tokens("100:", vec![Number(100.to_string()), Colon]);

        assert_scan_tokens(
            "2.4.8",
            vec![
                Number(2.to_string()), FullStop,
                Number(4.to_string()), FullStop,
                Number(8.to_string()),
            ],
        );
        assert_scan_tokens(
            "12:34-56.78",
            vec![
                Number(12.to_string()), Colon,
                Number(34.to_string()), Hyphen,
                Number(56.to_string()), FullStop,
                Number(78.to_string()),
            ],
        );

        assert_scan_tokens(
            "-99.77:55.33:11-",
            vec![
                Hyphen,
                Number(99.to_string()), FullStop,
                Number(77.to_string()), Colon,
                Number(55.to_string()), FullStop,
                Number(33.to_string()), Colon,
                Number(11.to_string()), Hyphen,
            ],
        );
    }

    #[test]
    fn scan_time() {
        assert_scan_tokens(
            "12.34",
            vec![Number(12.to_string()), FullStop, Number(34.to_string())],
        );
        assert_scan_tokens(
            "12:34.567",
            vec![
                Number(12.to_string()), Colon,
                Number(34.to_string()), FullStop,
                Number(567.to_string()),
            ],
        );
        assert_scan_tokens(
            "12:34:56.7890",
            vec![
                Number(12.to_string()), Colon,
                Number(34.to_string()), Colon,
                Number(56.to_string()), FullStop,
                Number(7890.to_string()),
            ],
        );
        assert_scan_tokens(
            "-9876:54:32.10",
            vec![
                Hyphen,
                Number(9876.to_string()), Colon,
                Number(54.to_string()), Colon,
                Number(32.to_string()), FullStop,
                Number(10.to_string()),
            ],
        );
    }

    #[test]
    fn scan_unknown_tokens() {
        assert!(Lexer::new(" ").scan().is_err());
        assert!(Lexer::new(" ").scan().is_err());
        assert!(Lexer::new("!").scan().is_err());
        assert!(Lexer::new("@").scan().is_err());
        assert!(Lexer::new("a").scan().is_err());
        assert!(Lexer::new("bcd").scan().is_err());
        assert!(Lexer::new("-e").scan().is_err());
        assert!(Lexer::new("f-").scan().is_err());
        assert!(Lexer::new(":g").scan().is_err());
        assert!(Lexer::new("g:").scan().is_err());
        assert!(Lexer::new(".i").scan().is_err());
        assert!(Lexer::new("j.").scan().is_err());
        assert!(Lexer::new("1-2-k").scan().is_err());
        assert!(Lexer::new("l-3-4").scan().is_err());
        assert!(Lexer::new("5:6:m").scan().is_err());
        assert!(Lexer::new("n:7:8").scan().is_err());
        assert!(Lexer::new("9.0.o").scan().is_err());
        assert!(Lexer::new("p.1.2").scan().is_err());
        assert!(Lexer::new("-3qrs4:5tuv6:7wxy8.999").scan().is_err());
        assert!(Lexer::new("-12:34:56.789z").scan().is_err());
    }

    #[test]
    fn parse_time_valid() {
        // Zero.
        assert_parse_time("0s", Time::builder().build());
        assert_parse_time("00s", Time::builder().build());
        assert_parse_time("0.0s", Time::builder().build());
        assert_parse_time("0.000000000s", Time::builder().build());
        assert_parse_time("00:00", Time::builder().build());
        assert_parse_time("00:00.0", Time::builder().build());
        assert_parse_time("00:00.000000000", Time::builder().build());
        assert_parse_time("0:00:00", Time::builder().build());
        assert_parse_time("0:00:00.0", Time::builder().build());
        assert_parse_time("00:00:00.000000000", Time::builder().build());
        assert_parse_time("-0s", Time::builder().build());
        assert_parse_time("-00s", Time::builder().build());
        assert_parse_time("-0.0s", Time::builder().build());
        assert_parse_time("-0.000000000s", Time::builder().build());
        assert_parse_time("-00:00", Time::builder().build());
        assert_parse_time("-00:00.0", Time::builder().build());
        assert_parse_time("-00:00.000000000", Time::builder().build());
        assert_parse_time("-0:00:00", Time::builder().build());
        assert_parse_time("-0:00:00.0", Time::builder().build());
        assert_parse_time("-00:00:00.000000000", Time::builder().build());

        assert_parse_time("0:00:00.000000001", Time::builder().nanoseconds(1).build());
        assert_parse_time("0:00:00.1", Time::builder().nanoseconds(100000000).build());
        assert_parse_time("0:00:00.100", Time::builder().nanoseconds(100000000).build());
        assert_parse_time("0:00:00.987654", Time::builder().nanoseconds(987654000).build());
        assert_parse_time("0:00:00.999999999", Time::builder().nanoseconds(999999999).build());

        assert_parse_time("01s", Time::builder().seconds(1).build());
        assert_parse_time("59s", Time::builder().seconds(59).build());
        assert_parse_time("00:01", Time::builder().seconds(1).build());
        assert_parse_time("00:59", Time::builder().seconds(59).build());
        assert_parse_time("0:00:01", Time::builder().seconds(1).build());
        assert_parse_time("0:00:59", Time::builder().seconds(59).build());

        assert_parse_time("0:01:00", Time::builder().minutes(1).build());
        assert_parse_time("0:59:00", Time::builder().minutes(59).build());

        assert_parse_time("1:00:00", Time::builder().hours(1).build());
        assert_parse_time("99:00:00", Time::builder().hours(99).build());

        assert_parse_time("1:01:01", Time::builder().hours(1).minutes(1).seconds(1).build());
        assert_parse_time("12:34:56", Time::builder().hours(12).minutes(34).seconds(56).build());
        assert_parse_time("98765:43:21", Time::builder().hours(98765).minutes(43).seconds(21).build());
        assert_parse_time("19:28:37.465", Time::builder().hours(19).minutes(28).seconds(37).nanoseconds(465000000).build());

        // Seconds only.
        assert_parse_time("1s", Time::builder().seconds(1).build());
        assert_parse_time("59s", Time::builder().seconds(59).build());
        assert_parse_time("1.000000001s", Time::builder().seconds(1).nanoseconds(1).build());
        assert_parse_time("59.999999999s", Time::builder().seconds(59).nanoseconds(999999999).build());

        // Mintues and seconds only.
        assert_parse_time("00:01", Time::builder().seconds(1).build());
        assert_parse_time("00:59", Time::builder().seconds(59).build());
        assert_parse_time("01:00", Time::builder().minutes(1).build());
        assert_parse_time("59:00", Time::builder().minutes(59).build());
        assert_parse_time("59:59", Time::builder().minutes(59).seconds(59).build());
        assert_parse_time("12:34.56789", Time::builder().minutes(12).seconds(34).nanoseconds(567890000).build());

        // Negative.
        assert_parse_time("-0.000000001s", Time::builder().negative().nanoseconds(1).build());
        assert_parse_time("-0.999999999s", Time::builder().negative().nanoseconds(999999999).build());
        assert_parse_time("-1s", Time::builder().negative().seconds(1).build());
        assert_parse_time("-59s", Time::builder().negative().seconds(59).build());
        assert_parse_time("-01:00", Time::builder().negative().minutes(1).build());
        assert_parse_time("-59:00", Time::builder().negative().minutes(59).build());
        assert_parse_time("-01:00:00", Time::builder().negative().hours(1).build());
        assert_parse_time("-59:00:00", Time::builder().negative().hours(59).build());
        assert_parse_time("-11.22s", Time::builder().negative().seconds(11).nanoseconds(220000000).build());
        assert_parse_time("-11:22", Time::builder().negative().minutes(11).seconds(22).build());
        assert_parse_time("-11:22:33.456789", Time::builder().negative().hours(11).minutes(22).seconds(33).nanoseconds(456789000).build());
    }

    #[test]
    fn parse_invalid_time() {
        // Missing components.
        assert!(parse_time("").is_err());
        assert!(parse_time("12").is_err());
        assert!(parse_time("12:").is_err());
        assert!(parse_time("12:34:").is_err());
        assert!(parse_time(":34:56").is_err());
        assert!(parse_time("12::56").is_err());
        assert!(parse_time("-12").is_err());
        assert!(parse_time("-:34:56").is_err());
        assert!(parse_time("-12::56").is_err());

        // Too many components.
        assert!(parse_time("10:20:30:40").is_err());
        assert!(parse_time("10:20:30:40:50.60").is_err());

        // Trailing decimal.
        assert!(parse_time("12:34:56.").is_err());
        // Too many fractional second digits.
        assert!(parse_time("12:34:56.0123456789").is_err());

        // Invalid seconds.
        assert!(parse_time("00:00:0").is_err());
        assert!(parse_time("00:00:4").is_err());
        assert!(parse_time("00:00:60").is_err());
        assert!(parse_time("00:00:99").is_err());

        // Invalid minutes.
        assert!(parse_time("00:8:00").is_err());
        assert!(parse_time("00:60:00").is_err());
        assert!(parse_time("00:99:00").is_err());

        // Invalid seconds identifier.
        assert!(parse_time("1.2s4").is_err());
        assert!(parse_time("24:35s").is_err());
        assert!(parse_time("54:32.4s").is_err());
        assert!(parse_time("s4").is_err());

        // Invalid tokens.
        assert!(parse_time("00.00:00").is_err());
        assert!(parse_time("00:00-00").is_err());
        assert!(parse_time("00:00:00:").is_err());
        assert!(parse_time(":00:00:00").is_err());
        assert!(parse_time(".00:00:00").is_err());
        assert!(parse_time("00:00:00-").is_err());
        assert!(parse_time("00:00:00..").is_err());
        assert!(parse_time("--00:00:00").is_err());
    }

    fn assert_scan_tokens(input: &str, tokens: Vec<Token>) {
        assert_eq!(Lexer::new(input).scan().unwrap(), tokens);
    }

    fn assert_parse_time(time_str: &str, time: Time) {
        assert_eq!(parse_time(time_str).unwrap(), time);
    }
}
