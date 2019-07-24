use crate::calc::parse::parse_expression;
use crate::calc::parse::BinaryOp;
use crate::calc::parse::Expr;
use crate::calc::parse::Literal;
use crate::calc::parse::ParseError;
use crate::calc::parse::UnaryOp;
use crate::time::Time;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

impl Expr {
    fn accept<T>(&self, visitor: &ExprVisitor<Result = T>) -> T {
        match *self {
            Expr::Literal(_) => visitor.visit_literal(self),
            Expr::Unary(_, _) => visitor.visit_unary(self),
            Expr::Binary(_, _, _) => visitor.visit_binary(self),
        }
    }
}

trait ExprVisitor {
    type Result;
    fn visit_literal(&self, expr: &Expr) -> Self::Result;
    fn visit_binary(&self, expr: &Expr) -> Self::Result;
    fn visit_unary(&self, expr: &Expr) -> Self::Result;
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum EvalResult {
    Time(Time),
    Number(Decimal),
}

impl std::fmt::Display for EvalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            EvalResult::Time(t) => write!(f, "{}", t),
            EvalResult::Number(n) => write!(f, "{}", n),
        }
    }
}

struct ExprEvaluator;

#[derive(Debug)]
pub(crate) enum EvalError {
    ParseError(ParseError),
    MultiplyTimes,
    AddTimeAndNumber,
    SubtractTimeAndNumber,
    DivideNumberByTime,
}

impl std::convert::From<ParseError> for EvalError {
    fn from(parse_error: ParseError) -> Self {
        EvalError::ParseError(parse_error)
    }
}

impl ExprVisitor for ExprEvaluator {
    type Result = Result<EvalResult, EvalError>;

    fn visit_literal(&self, expr: &Expr) -> Result<EvalResult, EvalError> {
        match expr {
            Expr::Literal(Literal::Time(t)) => Result::Ok(EvalResult::Time(*t)),
            Expr::Literal(Literal::Number(n)) => Result::Ok(EvalResult::Number(*n)),
            _ => panic!(),
        }
    }

    fn visit_binary(&self, expr: &Expr) -> Result<EvalResult, EvalError> {
        match expr {
            Expr::Binary(left, op, right) => {
                let r1 = left.accept(self)?;
                let r2 = right.accept(self)?;

                match (r1, r2) {
                    (EvalResult::Number(n1), EvalResult::Number(n2)) => match op {
                        BinaryOp::Add => Result::Ok(EvalResult::Number(n1 + n2)),
                        BinaryOp::Subtract => Result::Ok(EvalResult::Number(n1 - n2)),
                        BinaryOp::Multiply => Result::Ok(EvalResult::Number(n1 * n2)),
                        BinaryOp::Divide => Result::Ok(EvalResult::Number(n1 / n2)),
                    },
                    (EvalResult::Time(t1), EvalResult::Time(t2)) => match op {
                        BinaryOp::Add => Result::Ok(EvalResult::Time(t1 + t2)),
                        BinaryOp::Subtract => Result::Ok(EvalResult::Time(t1 - t2)),
                        BinaryOp::Divide => Result::Ok(EvalResult::Number(t1 / t2)),
                        BinaryOp::Multiply => Result::Err(EvalError::MultiplyTimes),
                    },
                    (EvalResult::Time(t), EvalResult::Number(n)) => match op {
                        BinaryOp::Multiply => Result::Ok(EvalResult::Time(t * n)),
                        BinaryOp::Divide => Result::Ok(EvalResult::Time(t / n)),
                        BinaryOp::Add => Result::Err(EvalError::AddTimeAndNumber),
                        BinaryOp::Subtract => Result::Err(EvalError::SubtractTimeAndNumber),
                    },
                    (EvalResult::Number(n), EvalResult::Time(t)) => match op {
                        BinaryOp::Multiply => Result::Ok(EvalResult::Time(n * t)),
                        BinaryOp::Add => Result::Err(EvalError::AddTimeAndNumber),
                        BinaryOp::Subtract => Result::Err(EvalError::SubtractTimeAndNumber),
                        BinaryOp::Divide => Result::Err(EvalError::DivideNumberByTime),
                    },
                }
            }
            _ => panic!(),
        }
    }

    fn visit_unary(&self, expr: &Expr) -> Result<EvalResult, EvalError> {
        match expr {
            Expr::Unary(UnaryOp::Negative, operand_expr) => {
                let operand = operand_expr.accept(self)?;
                match operand {
                    EvalResult::Time(t) => Result::Ok(EvalResult::Time(t * dec!(-1))),
                    EvalResult::Number(n) => Result::Ok(EvalResult::Number(n * dec!(-1))),
                }
            }
            _ => panic!(),
        }
    }
}

pub(in crate) fn eval(expression: &str) -> Result<EvalResult, EvalError> {
    parse_expression(expression)?.accept(&ExprEvaluator)
}

#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use super::eval;
    use crate::calc::eval::EvalResult;
    use crate::time::Time;
    use rust_decimal_macros::dec;

    #[test]
    fn eval_numbers_only() {
        assert_eval("2468.13579n", EvalResult::Number(dec!(2468.13579)));

        assert_eval("1n + 2n", EvalResult::Number(dec!(3)));
        assert_eval("3n-4n", EvalResult::Number(dec!(-1)));
        assert_eval("5n * -6n", EvalResult::Number(dec!(-30)));
        assert_eval("-7.7n/-8.8n", EvalResult::Number(dec!(0.875)));

        assert_eval("2n + 4n * 6n", EvalResult::Number(dec!(26)));
        assert_eval("2n + (4n * 6n)", EvalResult::Number(dec!(26)));
        assert_eval("(2n + 4n) * 6n", EvalResult::Number(dec!(36)));
        assert_eval("-1n - 3n / 6n * 7n + 9n", EvalResult::Number(dec!(4.5)));
        assert_eval("(-1n - 3n / 6n) * (7n + 9n)", EvalResult::Number(dec!(-24)));
        assert_eval("1n / -2n * -(3n-4n) - (5n * 6n + 7n) + 8n*9n", EvalResult::Number(dec!(34.5)));
    }

    #[test]
    fn eval_times_only() {
        assert_eval("8:04:02.13579",
            EvalResult::Time(Time::builder().hours(8).minutes(4).seconds(2).nanoseconds(135790000).build()));
        assert_eval("1:23:45 + 5:43:21",
            EvalResult::Time(Time::builder().hours(7).minutes(7).seconds(6).build()));
        assert_eval("0:11:22-0:33:44",
            EvalResult::Time(Time::builder().negative().minutes(22).seconds(22).build()));
        assert_eval("4:44:44 / 5:55:55", EvalResult::Number(dec!(0.8)));


        assert_eval("1:00:00 - 0:00:01 + 59:00:00",
            EvalResult::Time(Time::builder().hours(59).minutes(59).seconds(59).build()));
        assert_eval("1:00:00 - (0:00:01 + 59:00:00)",
            EvalResult::Time(Time::builder().negative().hours(58).seconds(1).build()));
        assert_eval("(1:00:00 - 0:00:01) + 59:00:00",
            EvalResult::Time(Time::builder().hours(59).minutes(59).seconds(59).build()));

        assert_eval("1:00:00 * (0:00:30 / 0:01:00)",
            EvalResult::Time(Time::builder().minutes(30).build()));
        assert_eval("(2:22:22 / 1:11:11) + (3:33:33/1:11:11)", EvalResult::Number(dec!(5)));

        assert_eval("((-(0:40:40 - 0:29:29) / 0:22:22) + 2n) * (1:00:00/0:30:00) * 0:15:15",
            EvalResult::Time(Time::builder().minutes(45).seconds(45).build()));
    }

    #[test]
    fn eval_numbers_and_times() {
        assert_eval("0:30:00 * 1.5n", EvalResult::Time(Time::builder().minutes(45).build()));
        assert_eval("2n * 1:11:11",
            EvalResult::Time(Time::builder().hours(2).minutes(22).seconds(22).build()));
        assert_eval("12:24:48 / 2n",
            EvalResult::Time(Time::builder().hours(6).minutes(12).seconds(24).build()));

        assert_eval("1:00:00 + 1.5n * 0:30:00",
            EvalResult::Time(Time::builder().hours(1).minutes(45).build()));
        assert_eval("0:12:34 * 2n / 0:25:08", EvalResult::Number(dec!(1)));

        assert_eval("((24:00:00 - 4n * 2:30:00) / 14n - (1:30:00 + 0:30:00)) / -0:30:00 ",
            EvalResult::Number(dec!(2)));

        assert_eval("(((8:00:00 * 3n) / 6:00:00) + 10n) * (0:12:00 / 0:08:00) + 4n",
            EvalResult::Number(dec!(25)));
    }

    #[test]
    fn eval_invalid() {
        assert!(eval("1n + 0:00:02").is_err());
        assert!(eval("0:00:30 + 4n").is_err());
        assert!(eval("5n - 0:06:00").is_err());
        assert!(eval("7:00:00 - 8n").is_err());
        assert!(eval("9:09:09 * 10:10:10").is_err());
        assert!(eval("11n / 12:12:12").is_err());
    }

    fn assert_eval(expr: &str, result: EvalResult) {
        assert_eq!(eval(expr).unwrap(), result)
    }
}
