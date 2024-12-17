
use crate::Day;
use regex::Regex;

type Int = u64;

pub mod runtime { }

pub mod tokens {
    use super::*;
    
    use logos::{Lexer, Logos, Span};

    #[derive(Debug, Logos)]
    pub enum Token {
        // keywords
        #[token("do")]
        KwDo,
        #[token("don't")]
        KwDont,

        // punctuation
        #[token("(")]
        LeftParenthesis,
        #[token(")")]
        RightParenthesis,
        #[token(",")]
        Comma,
        
        // values
        #[regex(r"\d{1,3}", |token| token.slice().parse::<Int>().unwrap())]
        Int(Int),
    }
}

pub mod ast {
    use std::{iter::Sum, str::FromStr};
    use evaluator::{Mode, TryMul};
    use std::rc::Rc;
    use nom::{
        branch::alt, bytes::complete::tag, character::complete::digit1, combinator::map, sequence::tuple, IResult
    };

    use super::*;

    #[derive(Clone, Debug)]
    pub enum Value {
        Int(Int),
        Void,
    }

    impl std::fmt::Display for Value {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Value::Int(number) => number.fmt(f),
                Value::Void => write!(f, "void"),
            }
        }
    }

    impl Sum for Value {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            Value::Int(iter.filter_map(
                |value| match value {
                    Value::Int(number) => Some(number),
                    Value::Void => None,
                })
                .sum()
            )
        }
    }

    pub enum Stmt {
        Evaluate(Expr),
        SetMode(evaluator::Mode),
    }

    impl Stmt {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            let set_mode_parser = map(
                alt((
                    tag("do()"),
                    tag("don't()")
                )),
                |tag| {
                    match tag {
                        "do()" => Stmt::SetMode(Mode::Eval),
                        "don't()" => Stmt::SetMode(Mode::Ignore),
                        _ => unreachable!(),
                    }
                }
            );

            alt((set_mode_parser, map(Expr::parse, Stmt::Evaluate)))(input)
        }
    }

    impl FromStr for Stmt {
        type Err = eyre::Report;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match Self::parse(input) {
                Ok((remaining, expr)) => {
                    if remaining.trim().is_empty() {
                        Ok(expr)
                    } else {
                        Err(eyre::eyre!("unexpected input remaining: {}", remaining))
                    }
                }
                Err(err) => Err(eyre::eyre!("failed to parse expression: {:?}", err)),
            }
        }
    }

    pub enum Expr {
        Literal(Value),
        Mul{ lhs: Rc<Expr>, rhs: Rc<Expr> },
    }

    impl Expr {
        pub fn parse(input: &str) -> IResult<&str, Self> {
            let int_parser = map(digit1, |digits: &str| {
                Expr::Literal(Value::Int(digits.parse::<Int>().unwrap()))
            });
    
            // Parser for multiplication expressions
            let mul_parser = map(
                tuple((
                    tag("mul("),
                    Expr::parse,
                    tag(","),
                    Expr::parse,
                    tag(")"),
                )),
                |(_, lhs, _, rhs, _)| Expr::Mul {
                    lhs: Rc::new(lhs),
                    rhs: Rc::new(rhs),
                },
            );

            // Combine parsers to handle either integers or multiplication expressions
            alt((mul_parser, int_parser))(input)
        }
    }

    impl FromStr for Expr {
        type Err = eyre::Report;

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match Self::parse(input) {
                Ok((remaining, expr)) => {
                    if remaining.trim().is_empty() {
                        Ok(expr)
                    } else {
                        Err(eyre::eyre!("unexpected input remaining: {}", remaining))
                    }
                }
                Err(err) => Err(eyre::eyre!("failed to parse expression: {:?}", err)),
            }
        }
    }

    impl evaluator::TryMul for Value {
        fn try_mul(&self, other: &Self) -> eyre::Result<Self> {
            match (self, other) {
                (Value::Int(lhs), Value::Int(rhs)) => Ok(Value::Int(lhs * rhs)),
                (lhs, rhs) => Err(eyre::eyre!("mul is undefined for args: {:?}, {:?}", lhs, rhs))
            }
        }
    }

    impl Expr {
        pub fn eval(&self) -> eyre::Result<Value> {
            match self {
                Expr::Mul { lhs, rhs } => lhs.eval()?.try_mul(&rhs.eval()?),
                Expr::Literal(value) => Ok(value.clone()),
            }
        }
    }
}

pub fn run(program: &str) -> ast::Value {
    let regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut evaluator = evaluator::Evaluator::default();

    regex.find_iter(program)
        .map(|needle| needle.as_str()
            .parse::<ast::Stmt>()
            .expect("parsing successful")
        )
        .map(|expr| evaluator
            .process(expr)
            .expect("statement processing successful")
        )
        .sum::<ast::Value>()

}

pub mod part1 {
    use super::*;

    pub fn solve() {
        let day = Day::new(3, 1);
        let result = run(&day.input());
        day.report(result);
    }
}


pub mod part2 {
    use super::*;
    
    pub fn solve() {
        let day = Day::new(3, 2);
        let result = run(&day.input());
        day.report(result);
    }
}