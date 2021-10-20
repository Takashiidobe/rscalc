use nom::branch::alt;
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
}

pub fn evaluate(expr: Expr) -> f64 {
    use Expr::*;
    match expr {
        Num(num) => num,
        Add(e1, e2) => evaluate(*e1) + evaluate(*e2),
        Sub(e1, e2) => evaluate(*e1) - evaluate(*e2),
        Mul(e1, e2) => evaluate(*e1) * evaluate(*e2),
        Div(e1, e2) => evaluate(*e1) / evaluate(*e2),
        Pow(e1, e2) => evaluate(*e1).powf(evaluate(*e2)),
    }
}

fn parse_num_str(num: &str) -> Expr {
    let num = f64::from_str(num).unwrap();
    Expr::Num(num)
}

fn parse_number(input: &str) -> IResult<&str, Expr> {
    map(delimited(space0, digit1, space0), parse_num_str)(input)
}

fn parse(input: &str) -> IResult<&str, Expr> {
    parse_basic_expr(input)
}

fn parse_basic_expr(input: &str) -> IResult<&str, Expr> {
    parse_math_expr(input)
}

fn parse_parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        space0,
        delimited(char('('), parse_math_expr, char(')')),
        space0,
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expr> {
    alt((parse_parens, parse_number))(input)
}

fn parse_factor(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_operation(input)?;
    let (input, exprs) = many0(tuple((char('^'), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_term(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_factor(input)?;
    let (input, exprs) = many0(tuple((alt((char('/'), char('*'))), parse_factor)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_math_expr(input: &str) -> IResult<&str, Expr> {
    let (input, num1) = parse_term(input)?;
    let (input, exprs) = many0(tuple((alt((char('+'), char('-'))), parse_term)))(input)?;
    Ok((input, parse_expr(num1, exprs)))
}

fn parse_expr(expr: Expr, rem: Vec<(char, Expr)>) -> Expr {
    rem.into_iter().fold(expr, |acc, val| parse_op(val, acc))
}

fn parse_op(tup: (char, Expr), e1: Expr) -> Expr {
    use Expr::*;

    let (op, e2) = tup;
    match op {
        '+' => Add(Box::new(e1), Box::new(e2)),
        '-' => Sub(Box::new(e1), Box::new(e2)),
        '*' => Mul(Box::new(e1), Box::new(e2)),
        '/' => Div(Box::new(e1), Box::new(e2)),
        '^' => Pow(Box::new(e1), Box::new(e2)),
        _ => panic!("Unknown Operation"),
    }
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let (_, parsed) = parse(&line).unwrap();
                let result = evaluate(parsed);
                rl.add_history_entry(line.as_str());
                println!("{}", result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}
