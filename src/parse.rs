use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Token {
    Hyphen,
    Number(String),
    Colon,
    FullStop,
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
                    self.scan_character()
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

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;
    use super::Token::*;

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

    fn assert_scan_tokens(input: &str, tokens: Vec<Token>) {
        assert_eq!(Lexer::new(input).scan(), tokens);
    }

    fn assert_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) {
        let result = std::panic::catch_unwind(f);
        assert!(result.is_err());
    }
}
