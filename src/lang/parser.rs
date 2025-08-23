use super::ast;
use nom::IResult;
use nom::character::complete;

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
