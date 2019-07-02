use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use crate::time::Time;
use crate::time::TimeBuilder;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Token {
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

impl<'a> Lexer<'a> {
    fn new(input: &str) -> Lexer {
        Lexer {
            chars: input.chars().peekable(),
            tokens: Vec::new(),
            scan_complete: false,
        }
    }

    fn scan(&mut self) -> Vec<Token> {
        if !self.scan_complete {
            while let Some(ch) = self.peek() {
                if ch.is_numeric() {
                    self.scan_number();
                } else {
                    self.scan_character();
                }
            }
        }

        self.scan_complete = true;
        self.tokens.clone()
    }

    fn scan_number(&mut self) {
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
    }

    fn scan_character(&mut self) {
        let token = match self.next() {
            Option::Some('-') => Token::Hyphen,
            Option::Some(':') => Token::Colon,
            Option::Some('.') => Token::FullStop,
            Option::Some(c) => panic!("Unexpected character: {}", c),
            Option::None => panic!("Unexpected end of input."),
        };

        self.tokens.push(token);
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

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens }
    }

    fn parse(self) -> Time {
        let mut time_builder = Time::builder();
        let mut token_iter = self.tokens.into_iter().peekable();

        // Only consume first token if it is a hyphen, indicating a negative time.
        if token_iter.peek() == Option::Some(&Token::Hyphen) {
            token_iter.next();
            time_builder.negative();
        }

        match token_iter.next() {
            Option::Some(Token::Number(n)) => {time_builder.hours(n.parse().unwrap());},
            Option::Some(t) => panic!("Expected hours, but found {}.", t),
            Option::None => panic!("Expected hours, but found reached end of time."),
        }
        match token_iter.next() {
            Option::Some(Token::Colon) => (),
            Option::Some(t) => panic!("Expected :, but found {}.", t),
            Option::None => panic!("Expected :, but found reached end of time."),
        }

        match token_iter.next() {
            Option::Some(Token::Number(n)) => {
                if n.len() != 2 {
                    panic!("Expected minutes in 2 digit format.");
                }
                time_builder.minutes(n.parse().unwrap());
            },
            Option::Some(t) => panic!("Expected minutes, but found {}.", t),
            Option::None => panic!("Expected minutes, but found reached end of time."),
        }
        match token_iter.next() {
            Option::Some(Token::Colon) => (),
            Option::Some(t) => panic!("Expected :, but found {}", t),
            Option::None => panic!("Expected :, but found reached end of time."),
        }

        match token_iter.next() {
            Option::Some(Token::Number(n)) => {
                if n.len() != 2 {
                    panic!("Expected seconds in 2 digit format.");
                }
                time_builder.seconds(n.parse().unwrap());
            },
            Option::Some(t) => panic!("Expected seconds, but found {}.", t),
            Option::None => panic!("Expected seconds, but found reached end of time."),
        }

        match token_iter.peek() {
            Option::Some(&Token::FullStop) => {
                token_iter.next();
                match token_iter.next() {
                    Option::Some(Token::Number(mut n)) => {
                        if n.len() > 9 {
                            panic!("Fractional seconds part is too large.");
                        }
                        while n.len() < 9 {
                            n.push('0');
                        }

                        time_builder.nanoseconds(n.parse().unwrap());
                    },
                    Option::Some(t) => panic!("Expected nanoseconds, but found {}.", t),
                    Option::None => panic!("Expected nanoseconds, but found end of time."),
                }
            },
            Option::Some(t) => panic!("Expected nanoseconds or end of time, but found {}.", t),
            Option::None => (),
        }

        time_builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Parser;
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
        assert_panic(|| Lexer::new(" ").scan());
        assert_panic(|| Lexer::new("!").scan());
        assert_panic(|| Lexer::new("@").scan());
        assert_panic(|| Lexer::new("a").scan());
        assert_panic(|| Lexer::new("bcd").scan());
        assert_panic(|| Lexer::new("-e").scan());
        assert_panic(|| Lexer::new("f-").scan());
        assert_panic(|| Lexer::new(":g").scan());
        assert_panic(|| Lexer::new("g:").scan());
        assert_panic(|| Lexer::new(".i").scan());
        assert_panic(|| Lexer::new("j.").scan());
        assert_panic(|| Lexer::new("1-2-k").scan());
        assert_panic(|| Lexer::new("l-3-4").scan());
        assert_panic(|| Lexer::new("5:6:m").scan());
        assert_panic(|| Lexer::new("n:7:8").scan());
        assert_panic(|| Lexer::new("9.0.o").scan());
        assert_panic(|| Lexer::new("p.1.2").scan());
        assert_panic(|| Lexer::new("-3qrs4:5tuv6:7wxy8.999").scan());
        assert_panic(|| Lexer::new("-12:34:56.789z").scan());
    }

    #[test]
    fn parse_time() {
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
        assert_panic(|| parse(""));
        assert_panic(|| parse("12"));
        assert_panic(|| parse("12:"));
        assert_panic(|| parse("12:34"));
        assert_panic(|| parse("12:34:"));
        assert_panic(|| parse(":34:56"));
        assert_panic(|| parse("12::56"));
        assert_panic(|| parse("-12"));
        assert_panic(|| parse("-12:34"));
        assert_panic(|| parse("-:34:56"));
        assert_panic(|| parse("-12::56"));

        // Trailing decimal.
        assert_panic(|| parse("12:34:56."));
        // Too many fractional second digits.
        assert_panic(|| parse("12:34:56.0123456789"));


        // Invalid seconds.
        assert_panic(|| parse("00:00:0"));
        assert_panic(|| parse("00:00:4"));
        assert_panic(|| parse("00:00:60"));
        assert_panic(|| parse("00:00:99"));

        // Invalid minutes.
        assert_panic(|| parse("00:8:00"));
        assert_panic(|| parse("00:60:00"));
        assert_panic(|| parse("00:99:00"));

        // Invalid tokens.
        assert_panic(|| parse("00.00:00"));
        assert_panic(|| parse("00:00-00"));
        assert_panic(|| parse("00:00:00:"));
        assert_panic(|| parse(":00:00:00"));
        assert_panic(|| parse(".00:00:00"));
        assert_panic(|| parse("00:00:00-"));
        assert_panic(|| parse("00:00:00.."));
        assert_panic(|| parse("--00:00:00"));
    }

    fn assert_scan_tokens(input: &str, tokens: Vec<Token>) {
        assert_eq!(Lexer::new(input).scan(), tokens);
    }

    fn assert_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) {
        let result = std::panic::catch_unwind(f);
        assert!(result.is_err());
    }

    fn assert_parse_time(time_str: &str, time: Time) {
        assert_eq!(parse(time_str), time);
    }

    fn parse(time: &str) -> Time {
        Parser::new(Lexer::new(time).scan()).parse()
    }
}
