use super::ast;
use nom::character::complete;
use nom::{IResult, branch, combinator, sequence};
use nom::{Parser, character};

pub fn const_int_parser(s: &str) -> IResult<&str, ast::ConstInt> {
    let (rest, integer) = complete::digit1(s)?;
    let value: i32 = integer.parse().unwrap();
    Ok((rest, ast::ConstInt::new(value)))
}

#[test]
fn const_int_parser_test() {
    let (_, actual) = const_int_parser("123").unwrap();
    let expect = ast::ConstInt::new(123);
    assert_eq!(actual, expect);
}

pub fn expr_parser(s: &str) -> IResult<&str, ast::Expr> {
    const_int_parser(s).map(|(rest, integer)| (rest, ast::Expr::ConstInt(integer)))
}

pub fn paren_expr_parser(s: &str) -> IResult<&str, ast::Expr> {
    sequence::delimited(character::char('('), expr_parser, character::char(')')).parse(s)
}

#[test]
fn paren_expr_parser_test() {
    let (_, actual) = paren_expr_parser("(123)").unwrap();
    let expect = ast::Expr::ConstInt(ast::ConstInt::new(123));
    assert_eq!(actual, expect);
}

pub fn primary_parser(s: &str) -> IResult<&str, ast::Expr> {
    branch::alt((
        combinator::map(const_int_parser, |const_int| ast::Expr::ConstInt(const_int)),
        paren_expr_parser,
    ))
    .parse(s)
}

#[test]
fn primary_parser_test1() {
    let (_, actual) = primary_parser("12").unwrap();
    let expect = ast::Expr::ConstInt(ast::ConstInt::new(12));
    assert_eq!(actual, expect);
}

#[test]
fn primary_parser_test2() {
    let (_, actual) = primary_parser("(345)").unwrap();
    let expect = ast::Expr::ConstInt(ast::ConstInt::new(345));
    assert_eq!(actual, expect);
}
