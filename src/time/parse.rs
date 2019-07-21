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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let t = match self {
            Token::Hyphen => String::from("-"),
            Token::Number(n) => n.clone(),
            Token::Colon => String::from(":"),
            Token::FullStop => String::from("."),
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
                } else {
                    if let Result::Err(err) = self.scan_character() {
                        errors.push(err);
                    }
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
    tokens: Vec<Token>
}

#[derive(Debug)]
pub enum ParseError {
    LexError(Vec<LexError>),
    ExpectedHours(Option<Token>),
    ExpectedMinutes(Option<Token>),
    ExpectedSeconds(Option<Token>),
    ExpectedNanoseconds(Option<Token>),
    ExpectedPeriod(Option<Token>),
    ExpectedColon(Option<Token>),
    ExpectedTwoDigitMinutes(Option<Token>),
    ExpectedTwoDigitSeconds(Option<Token>),
    SecondsOutOfRange(Option<Token>),
    MinutesOutOfRange(Option<Token>),
    FractionalSecondsTooLarge(Option<Token>),
    ExpectedEndOfInputOrFraction(Option<Token>),
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
        let mut time_builder = Time::builder();
        let mut token_iter = self.tokens.into_iter().peekable();

        // Only consume first token if it is a hyphen, indicating a negative time.
        if token_iter.peek() == Option::Some(&Token::Hyphen) {
            token_iter.next();
            time_builder.negative();
        }

        match token_iter.next() {
            Option::Some(Token::Number(n)) => {time_builder.hours(n.parse().unwrap());},
            Option::Some(t) => return Result::Err(ParseError::ExpectedHours(Option::Some(t))),
            Option::None => return Result::Err(ParseError::ExpectedHours(Option::None)),
        }
        match token_iter.next() {
            Option::Some(Token::Colon) => (),
            Option::Some(t) => return Result::Err(ParseError::ExpectedColon(Option::Some(t))),
            Option::None => return Result::Err(ParseError::ExpectedColon(Option::None)),
        }

        match token_iter.next() {
            Option::Some(Token::Number(n)) => {
                if let Result::Ok(minutes) = n.parse() {
                    if n.len() != 2 {
                        return Result::Err(ParseError::ExpectedTwoDigitMinutes(Option::Some(Token::Number(n))));
                    }
                    if minutes < 60 {
                        time_builder.minutes(minutes);
                    }
                    else {
                        return Result::Err(ParseError::MinutesOutOfRange(Option::Some(Token::Number(n))));
                    }
                }
                else {
                    return Result::Err(ParseError::ExpectedMinutes(Option::Some(Token::Number(n))));
                }
                // time_builder.minutes(n.parse().unwrap());
            },
            Option::Some(t) => return Result::Err(ParseError::ExpectedMinutes(Option::Some(t))),
            Option::None => return Result::Err(ParseError::ExpectedMinutes(Option::None)),
        }
        match token_iter.next() {
            Option::Some(Token::Colon) => (),
            Option::Some(t) => return Result::Err(ParseError::ExpectedColon(Option::Some(t))),
            Option::None => return Result::Err(ParseError::ExpectedColon(Option::None)),
        }

        match token_iter.next() {
            Option::Some(Token::Number(n)) => {
                if let Result::Ok(seconds) = n.parse() {
                    if n.len() != 2 {
                        return Result::Err(ParseError::ExpectedTwoDigitSeconds(Option::Some(Token::Number(n))));
                    }
                    if seconds < 60 {
                        time_builder.seconds(seconds);
                    }
                    else {
                        return Result::Err(ParseError::SecondsOutOfRange(Option::Some(Token::Number(n))));
                    }
                }
                else {
                    return Result::Err(ParseError::ExpectedSeconds(Option::Some(Token::Number(n))));
                }
                // time_builder.seconds(n.parse().unwrap());
            },
            Option::Some(t) => return Result::Err(ParseError::ExpectedSeconds(Option::Some(t))),
            Option::None => return Result::Err(ParseError::ExpectedSeconds(Option::None)),
        }

        match token_iter.peek() {
            Option::Some(&Token::FullStop) => {
                token_iter.next();
                match token_iter.next() {
                    Option::Some(Token::Number(mut n)) => {
                        if n.len() > 9 {
                            return Result::Err(ParseError::FractionalSecondsTooLarge(Option::Some(Token::Number(n))));
                        }
                        while n.len() < 9 {
                            n.push('0');
                        }

                        time_builder.nanoseconds(n.parse().unwrap());
                    },
                    Option::Some(t) => return Result::Err(ParseError::ExpectedNanoseconds(Option::Some(t))),
                    Option::None => return Result::Err(ParseError::ExpectedNanoseconds(Option::None)),
                }
            },
            Option::Some(t) => return Result::Err(ParseError::ExpectedEndOfInputOrFraction(Option::Some(t.clone()))),
            Option::None => (),
        }

        Result::Ok(time_builder.build())
    }
}

pub(crate) fn parse_time(time: &str) -> Result<Time, ParseError> {
    Parser::new(Lexer::new(time).scan()?).parse()
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;
    use super::Token::*;
    use super::parse_time;
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
        assert_parse_time("0:00:00", Time::builder().build());
        assert_parse_time("0:00:00.0", Time::builder().build());
        assert_parse_time("00:00:00.000000000", Time::builder().build());
        assert_parse_time("-0:00:00", Time::builder().build());
        assert_parse_time("-0:00:00.0", Time::builder().build());
        assert_parse_time("-00:00:00.000000000", Time::builder().build());

        assert_parse_time("0:00:00.000000001", Time::builder().nanoseconds(1).build());
        assert_parse_time("0:00:00.1", Time::builder().nanoseconds(100000000).build());
        assert_parse_time("0:00:00.100", Time::builder().nanoseconds(100000000).build());
        assert_parse_time("0:00:00.987654", Time::builder().nanoseconds(987654000).build());
        assert_parse_time("0:00:00.999999999", Time::builder().nanoseconds(999999999).build());

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
    }

    #[test]
    fn parse_invalid_time() {
        // Missing components.
        assert!(parse_time("").is_err());
        assert!(parse_time("12").is_err());
        assert!(parse_time("12:").is_err());
        assert!(parse_time("12:34").is_err());
        assert!(parse_time("12:34:").is_err());
        assert!(parse_time(":34:56").is_err());
        assert!(parse_time("12::56").is_err());
        assert!(parse_time("-12").is_err());
        assert!(parse_time("-12:34").is_err());
        assert!(parse_time("-:34:56").is_err());
        assert!(parse_time("-12::56").is_err());

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
