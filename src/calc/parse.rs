use std::fmt::Error;
use std::fmt::Formatter;
use std::iter::Peekable;
use std::slice::Iter;
use std::str::Chars;
use std::str::FromStr;

use rust_decimal::Decimal;

use crate::time::Time;

#[derive(Clone, PartialEq, Eq)]
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

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Token::Time(t) => write!(f, "{}", t),
            Token::Number(n) => write!(f, "{}", n),
            Token::Plus => write!(f, "+"),
            Token::Hyphen => write!(f, "-"),
            Token::Slash => write!(f, "/"),
            Token::Asterisk => write!(f, "*"),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
        }
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        std::fmt::Display::fmt(self, f)
    }
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

#[derive(Debug, PartialEq, Eq)]
enum Expr {
    Literal(Literal),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
}

#[derive(Debug, PartialEq, Eq)]
enum Literal {
    Number(Decimal),
    Time(Time),
}

#[derive(Debug, PartialEq, Eq)]
enum UnaryOp {
    Negative,
}

#[derive(Debug, PartialEq, Eq)]
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct Parser<'a>{
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a Vec<Token>) -> Parser<'a> {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    fn parse(&mut self) -> Expr {
        let expr = self.expression();
        if self.peek().is_some() {
            panic!("Reached end of expression but found leftover tokens: {:?}",
                   self.remaining_tokens());
        }

        expr
    }

    fn expression(&mut self) -> Expr {
        self.addition()
    }

    fn addition(&mut self) -> Expr {
        let mut expr = self.multiplication();
        while let Option::Some(token) = self.peek() {
            if token == &&Token::Plus {
                self.next(); // Consume plus
                expr = Expr::Binary(Box::new(expr), BinaryOp::Add, Box::new(self.multiplication()));
            }
            else if token == &&Token::Hyphen {
                self.next(); // Consume hyphen.
                expr = Expr::Binary(Box::new(expr), BinaryOp::Subtract, Box::new(self.multiplication()));
            }
            else {
                break;
            }
        }
        expr
    }

    fn multiplication(&mut self) -> Expr {
        let mut expr = self.unary();
        while let Option::Some(token) = self.peek() {
            if token == &&Token::Asterisk {
                self.next(); // Consume asterisk.
                expr = Expr::Binary(Box::new(expr), BinaryOp::Multiply, Box::new(self.unary()));
            }
            else if token == &&Token::Slash  {
                self.next(); // Consume slash.
                expr = Expr::Binary(Box::new(expr), BinaryOp::Divide, Box::new(self.unary()));
            }
            else {
                break;
            }
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.peek() == Option::Some(&&Token::Hyphen) {
            self.next(); // Consume hyphen.
            Expr::Unary(UnaryOp::Negative, Box::new(self.value()))
        }
        else {
            self.value()
        }
    }

    fn value(&mut self) -> Expr {
        match self.next() {
            Option::Some(Token::Number(n)) => {
                let num = Decimal::from_str(n).unwrap();
                Expr::Literal(Literal::Number(num))
            },
            Option::Some(Token::Time(t)) => {
                let time = Time::from_str(t).unwrap();
                Expr::Literal(Literal::Time(time))
            },
            Option::Some(Token::LeftParen) => {
                let expr = self.expression();
                match self.next() {
                    Option::Some(Token::RightParen) => (),
                    Option::Some(t) => panic!("Expected ), but found {}.", t),
                    Option::None => panic!("Expected ), but found nothing."),
                }
                expr
            }
            Option::Some(token) => panic!("Expected literal, but found {}.", token),
            Option::None => panic!("Expected literal, but found nothing."),
        }
    }

    fn peek(&mut self) -> Option<&&Token> {
        self.tokens.peek()
    }

    fn next(&mut self) -> Option<&Token> {
        self.tokens.next()
    }

    fn remaining_tokens(&mut self) -> Vec<&Token> {
        let mut result = Vec::new();

        while let Option::Some(t) = self.tokens.next() {
            result.push(t);
        }

        result
    }
}

fn parse_expression(expr: &str) -> Expr {
    Parser::new(&Lexer::new(expr).scan()).parse()
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use super::Token;
    use super::Token::*;
    use super::Expr;
    use super::Literal;
    use super::parse_expression;
    use crate::calc::parse::BinaryOp;
    use crate::calc::parse::UnaryOp;
    use crate::time::Time;
    use rust_decimal_macros::dec;

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

    #[test]
    fn parse_simple_expression() {
        assert_parse_expression("1n + 2n",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(dec!(1)))),
                BinaryOp::Add,
                Box::new(Expr::Literal(Literal::Number(dec!(2))))
            )
        );

        assert_parse_expression("3n - 4n",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(dec!(3)))),
                BinaryOp::Subtract,
                Box::new(Expr::Literal(Literal::Number(dec!(4))))
            )
        );

        assert_parse_expression("5n * 6n",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(dec!(5)))),
                BinaryOp::Multiply,
                Box::new(Expr::Literal(Literal::Number(dec!(6))))
            )
        );

        assert_parse_expression("7n / 8n",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Number(dec!(7)))),
                BinaryOp::Divide,
                Box::new(Expr::Literal(Literal::Number(dec!(8))))
            )
        );

        assert_parse_expression("-9n",
            Expr::Unary(UnaryOp::Negative, Box::new(Expr::Literal(Literal::Number(dec!(9))))));


        assert_parse_expression("00:00:10 + 00:00:20",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Time(Time::builder().seconds(10).build()))),
                BinaryOp::Add,
                Box::new(Expr::Literal(Literal::Time(Time::builder().seconds(20).build())))
            )
        );

        assert_parse_expression("00:30:00 - 00:40:00",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Time(Time::builder().minutes(30).build()))),
                BinaryOp::Subtract,
                Box::new(Expr::Literal(Literal::Time(Time::builder().minutes(40).build())))
            )
        );

        assert_parse_expression("5:00:00 * 123.456n",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Time(Time::builder().hours(5).build()))),
                BinaryOp::Multiply,
                Box::new(Expr::Literal(Literal::Number(dec!(123.456))))
            )
        );

        assert_parse_expression("66:00:00 / 77:00:00",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Time(Time::builder().hours(66).build()))),
                BinaryOp::Divide,
                Box::new(Expr::Literal(Literal::Time(Time::builder().hours(77).build())))
            )
        );

        assert_parse_expression("8:08:08 / 987.654n",
            Expr::Binary(
                Box::new(Expr::Literal(Literal::Time(Time::builder().hours(8).minutes(8).seconds(8).build()))),
                BinaryOp::Divide,
                Box::new(Expr::Literal(Literal::Number(dec!(987.654))))
            )
        );

        assert_parse_expression("-9876:54:32.10",
            Expr::Unary(
                UnaryOp::Negative,
                Box::new(Expr::Literal(Literal::Time(
                    Time::builder().hours(9876).minutes(54).seconds(32).nanoseconds(100000000).build()
                )))
            )
        );
    }

    #[test]
    fn parse_complex_expression() {
        assert_parse_expression("1n+2n*3n-4n/5n",
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(dec!(1)))),
                    BinaryOp::Add,
                    Box::new(Expr::Binary(
                        Box::new(Expr::Literal(Literal::Number(dec!(2)))),
                        BinaryOp::Multiply,
                        Box::new(Expr::Literal(Literal::Number(dec!(3))))
                    ))
                )),
                BinaryOp::Subtract,
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Number(dec!(4)))),
                    BinaryOp::Divide,
                    Box::new(Expr::Literal(Literal::Number(dec!(5))))
                ))
            )
        );

        assert_parse_expression("1:00:00 - 0:22:22 / 0:30:00 * 4:04:04 + 5:55:55",
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Literal(Literal::Time(
                        Time::builder().hours(1).build()
                    ))),
                    BinaryOp::Subtract,
                    Box::new(Expr::Binary(
                        Box::new(Expr::Binary(
                            Box::new(Expr::Literal(Literal::Time(
                                Time::builder().minutes(22).seconds(22).build()
                            ))),
                            BinaryOp::Divide,
                            Box::new(Expr::Literal(Literal::Time(
                                Time::builder().minutes(30).build()
                            )))
                        )),
                        BinaryOp::Multiply,
                        Box::new(Expr::Literal(Literal::Time(
                            Time::builder().hours(4).minutes(4).seconds(4).build()
                        )))
                    ))
                )),
                BinaryOp::Add,
                Box::new(Expr::Literal(Literal::Time(
                    Time::builder().hours(5).minutes(55).seconds(55).build()
                )))
            )
        );

        assert_parse_expression(
            "((-10:10:10 + 0:22:22) * (3.33n / -4.44n)) - (5:00:00 + -0:06:06) / 7.77n",
            Expr::Binary(
                Box::new(Expr::Binary(
                    Box::new(Expr::Binary(
                        Box::new(Expr::Unary(
                            UnaryOp::Negative,
                            Box::new(Expr::Literal(Literal::Time(
                                Time::builder().hours(10).minutes(10).seconds(10).build()
                            )))
                        )),
                        BinaryOp::Add,
                        Box::new(Expr::Literal(Literal::Time(
                            Time::builder().minutes(22).seconds(22).build()
                        )))
                    )),
                    BinaryOp::Multiply,
                    Box::new(Expr::Binary(
                        Box::new(Expr::Literal(Literal::Number(dec!(3.33)))),
                        BinaryOp::Divide,
                        Box::new(Expr::Unary(
                            UnaryOp::Negative,
                            Box::new(Expr::Literal(Literal::Number(dec!(4.44))))
                        ))
                    ))
                )),
                BinaryOp::Subtract,
                Box::new(Expr::Binary(
                    Box::new(Expr::Binary(
                        Box::new(Expr::Literal(Literal::Time(
                            Time::builder().hours(5).build()
                        ))),
                        BinaryOp::Add,
                        Box::new(Expr::Unary(
                            UnaryOp::Negative,
                            Box::new(Expr::Literal(Literal::Time(
                                Time::builder().minutes(6).seconds(6).build()
                            )))
                        ))
                    )),
                    BinaryOp::Divide,
                    Box::new(Expr::Literal(Literal::Number(dec!(7.77))))
                ))
            )
        );
    }

    #[test]
    fn parse_invalid() {
        assert_panic(|| parse_expression("+ 1n"));
        assert_panic(|| parse_expression("20:00:02 -"));
        assert_panic(|| parse_expression("(3n * 4n"));
        assert_panic(|| parse_expression("5:55:55 / 6:06:06 +"));
        assert_panic(|| parse_expression("7n + 8:08:08 )"));
        assert_panic(|| parse_expression("0:09:09 * 10n ) + 11:11:11"));
    }

    fn assert_scan_tokens(input: &str, tokens: Vec<Token>) {
        assert_eq!(Lexer::new(input).scan(), tokens);
    }

    fn assert_parse_expression(input: &str, expr: Expr) {
        assert_eq!(parse_expression(input), expr);
    }

    fn assert_panic<F: FnOnce() -> R + std::panic::UnwindSafe, R>(f: F) {
        let result = std::panic::catch_unwind(f);
        assert!(result.is_err());
    }
}
