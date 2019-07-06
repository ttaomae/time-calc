use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Token {
    Time(String),
    Number(String),
    Plus,
    Hyphen,
    Slash,
    Asterisk,
    LeftParen,
    RightParen,
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    scan_complete: bool
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
                }
                else {
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
                Option::Some(c) if c.is_digit(10) || *c == ':' || *c == '.' => {
                    num.push(*c);
                    self.next();
                }
                _ => break,
            }
        }
        if self.peek() == Option::Some(&'n') {
            self.next();
            self.tokens.push(Token::Number(num));

        }
        else {
            self.tokens.push(Token::Time(num));
        }
    }

    fn scan_character(&mut self) {
        let token = match self.next() {
            // Skip whitespace.
            Option::Some(c) if c.is_whitespace() => return,
            Option::Some('+') => Token::Plus,
            Option::Some('-') => Token::Hyphen,
            Option::Some('/') => Token::Slash,
            Option::Some('*') => Token::Asterisk,
            Option::Some('(') => Token::LeftParen,
            Option::Some(')') => Token::RightParen,
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

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;
    use super::Token::*;

    #[test]
    fn scan_single_token() {
        // Single character token.
        assert_scan_tokens("+", vec![Plus]);
        assert_scan_tokens("-", vec![Hyphen]);
        assert_scan_tokens("/", vec![Slash]);
        assert_scan_tokens("*", vec![Asterisk]);
        assert_scan_tokens("(", vec![LeftParen]);
        assert_scan_tokens(")", vec![RightParen]);

        // Valid time.
        assert_scan_tokens("00:00:00", vec![Time("00:00:00".to_string())]);
        assert_scan_tokens("00:00:00.00", vec![Time("00:00:00.00".to_string())]);
        assert_scan_tokens("00:00:00.000000000", vec![Time("00:00:00.000000000".to_string())]);
        assert_scan_tokens("00:00:12", vec![Time("00:00:12".to_string())]);
        assert_scan_tokens("00:12:34", vec![Time("00:12:34".to_string())]);
        assert_scan_tokens("12:34:56", vec![Time("12:34:56".to_string())]);
        assert_scan_tokens("12:34:56.789", vec![Time("12:34:56.789".to_string())]);

        // Invalid time.
        assert_scan_tokens("00", vec![Time("00".to_string())]);
        assert_scan_tokens("00.00", vec![Time("00.00".to_string())]);
        assert_scan_tokens("00:00", vec![Time("00:00".to_string())]);
        assert_scan_tokens("00:00.00", vec![Time("00:00.00".to_string())]);

        assert_scan_tokens("60:60:60", vec![Time("60:60:60".to_string())]);
        assert_scan_tokens("98.76.54.321", vec![Time("98.76.54.321".to_string())]);

        // Valid numbers.
        assert_scan_tokens("0n", vec![Number(0.to_string())]);
        assert_scan_tokens("5n", vec![Number(5.to_string())]);
        assert_scan_tokens("9n", vec![Number(9.to_string())]);
        assert_scan_tokens("13579n", vec![Number(13579.to_string())]);
        assert_scan_tokens("13579.02468n", vec![Number(13579.02468.to_string())]);

        // Invalid numbers.
        assert_scan_tokens("123.456.789n", vec![Number("123.456.789".to_string())]);
        assert_scan_tokens("123:456:789n", vec![Number("123:456:789".to_string())]);
        assert_scan_tokens("123.456:789n", vec![Number("123.456:789".to_string())]);
        assert_scan_tokens("123:456.789n", vec![Number("123:456.789".to_string())]);
    }

    #[test]
    fn scan_multiple_tokens() {
        // Single character tokens.
        assert_scan_tokens("+-", vec![Plus, Hyphen]);
        assert_scan_tokens("/*", vec![Slash, Asterisk]);
        assert_scan_tokens("+-/*", vec![Plus, Hyphen, Slash, Asterisk]);
        assert_scan_tokens("*+/-", vec![Asterisk, Plus, Slash, Hyphen]);

        // Valid expressions.
        assert_scan_tokens("-123n", vec![Hyphen, Number("123".to_string())]);
        assert_scan_tokens("-97:53:10.2468", vec![Hyphen, Time("97:53:10.2468".to_string())]);
        assert_scan_tokens("11:11:11 + 11:11:11",
            vec![Time("11:11:11".to_string()), Plus, Time("11:11:11".to_string())]);
        assert_scan_tokens("22:22:22 - 22:22:22",
            vec![Time("22:22:22".to_string()), Hyphen, Time("22:22:22".to_string())]);
        assert_scan_tokens("33:33:33 * 33:33:33",
            vec![Time("33:33:33".to_string()), Asterisk, Time("33:33:33".to_string())]);
        assert_scan_tokens("44:44:44 / 44:44:44",
            vec![Time("44:44:44".to_string()), Slash, Time("44:44:44".to_string())]);
        assert_scan_tokens("55:55:55 / 5555.55n",
            vec![Time("55:55:55".to_string()), Slash, Number("5555.55".to_string())]);

        // Long expressions.
        assert_scan_tokens("11:11:11+22:22:22-33:33:33",
            vec![
                Time("11:11:11".to_string()), Plus,
                Time("22:22:22".to_string()), Hyphen,
                Time("33:33:33".to_string())
            ]
        );
        assert_scan_tokens("111.111n + 222.222n - 333.333n * 444.444n / 555.555n",
            vec![
                Number("111.111".to_string()), Plus,
                Number("222.222".to_string()), Hyphen,
                Number("333.333".to_string()), Asterisk,
                Number("444.444".to_string()), Slash,
                Number("555.555".to_string())
            ]
        );
        assert_scan_tokens("(11:11:11 - (22:22:22 + 33:33:33)) / 1234.5678n",
            vec![
                LeftParen,
                Time("11:11:11".to_string()), Hyphen, LeftParen,
                Time("22:22:22".to_string()), Plus,
                Time("33:33:33".to_string()), RightParen, RightParen, Slash,
                Number("1234.5678".to_string())
            ]
        );
        assert_scan_tokens("( (11:11:11 + 22:22:22) / (33:33:33 - 44:44:44) ) * 55:55:55",
            vec![
                LeftParen, LeftParen,
                Time("11:11:11".to_string()), Plus,
                Time("22:22:22".to_string()), RightParen, Slash, LeftParen,
                Time("33:33:33".to_string()), Hyphen,
                Time("44:44:44".to_string()), RightParen, RightParen, Asterisk,
                Time("55:55:55".to_string())
            ]
        );

        // Invalid expressions.
        assert_scan_tokens("11:22:33--", vec![Time("11:22:33".to_string()), Hyphen, Hyphen]);
        assert_scan_tokens("22:33:44 +", vec![Time("22:33:44".to_string()), Plus]);
        assert_scan_tokens("/ 33:44:55", vec![Slash, Time("33:44:55".to_string())]);
        assert_scan_tokens("66:55:44 + 123.456n",
            vec![Time("66:55:44".to_string()), Plus, Number("123.456".to_string())]);
        assert_scan_tokens("12:34:56 * 65:43:21",
            vec![Time("12:34:56".to_string()), Asterisk, Time("65:43:21".to_string())]);
    }

    fn assert_scan_tokens(input: &str, tokens: Vec<Token>) {
        assert_eq!(Lexer::new(input).scan(), tokens);
    }
}