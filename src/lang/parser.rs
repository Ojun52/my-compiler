use super::ast;
use nom::character::complete;
use nom::{IResult, sequence};
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
