use super::ast;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi;
use nom::{IResult, branch, combinator, sequence};

pub fn const_int_parser(s: &str) -> IResult<&str, ast::ConstInt> {
    let (rest, integer) = digit1(s)?;
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
    let op_kind_parser = combinator::map(branch::alt((tag("+"), tag("-"))), |op| match op {
        "+" => ast::OpKind::Add,
        "-" => ast::OpKind::Sub,
        _ => panic!("Expected + or -."),
    });

    let op_mul_parser = (op_kind_parser, mul_parser);
    let (rest, first_mul) = mul_parser(s)?;

    let (rest, op_primary_vec) = multi::many0(op_mul_parser).parse(rest)?;

    Ok((
        rest,
        op_primary_vec
            .iter()
            .fold(first_mul, |acc, (op_kind, primary)| {
                ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
                    op_kind.clone(),
                    acc,
                    primary.clone(),
                )))
            }),
    ))
}

pub fn paren_expr_parser(s: &str) -> IResult<&str, ast::Expr> {
    sequence::delimited(tag("("), expr_parser, tag(")")).parse(s)
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

pub fn mul_parser(s: &str) -> IResult<&str, ast::Expr> {
    let op_kind_parser = combinator::map(branch::alt((tag("*"), tag("/"))), |op| match op {
        "*" => ast::OpKind::Mul,
        "/" => ast::OpKind::Div,
        _ => panic!("Expected * or /."),
    });

    let op_primary_parser = (op_kind_parser, primary_parser);
    let (rest, first_primary) = primary_parser(s)?;

    let (rest, op_primary_vec) = multi::many0(op_primary_parser).parse(rest)?;

    Ok((
        rest,
        op_primary_vec
            .iter()
            .fold(first_primary, |acc, (op_kind, primary)| {
                ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
                    op_kind.clone(),
                    acc,
                    primary.clone(),
                )))
            }),
    ))
}

#[test]
pub fn mul_parser_test() {
    let (_, actual) = mul_parser("4*5/2").unwrap();

    let four_times_five = ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
        ast::OpKind::Mul,
        ast::Expr::ConstInt(ast::ConstInt::new(4)),
        ast::Expr::ConstInt(ast::ConstInt::new(5)),
    )));

    let expect = ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
        ast::OpKind::Div,
        four_times_five,
        ast::Expr::ConstInt(ast::ConstInt::new(2)),
    )));

    assert_eq!(actual, expect);
}
