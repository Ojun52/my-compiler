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
    equality_parser(s)
}

pub fn equality_parser(s: &str) -> IResult<&str, ast::Expr> {
    let op_kind_parser = combinator::map(branch::alt((tag("=="), tag("!="))), |op| match op {
        "==" => ast::OpKind::Equal,
        "!=" => ast::OpKind::NotEqual,
        _ => panic!("Expected == or !=."),
    });

    let op_relational_parser = (op_kind_parser, relational_parser);
    let (rest, first_relational) = relational_parser(s)?;

    let (rest, op_relational_vec) = multi::many0(op_relational_parser).parse(rest)?;
    Ok((
        rest,
        op_relational_vec
            .iter()
            .fold(first_relational, |acc, (op_kind, primary)| {
                ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
                    op_kind.clone(),
                    acc,
                    primary.clone(),
                )))
            }),
    ))
}

pub fn relational_parser(s: &str) -> IResult<&str, ast::Expr> {
    let op_kind_parser = combinator::map(
        branch::alt((tag("<="), tag(">="), tag(">"), tag("<"))),
        |op| match op {
            "<" => ast::OpKind::Less,
            "<=" => ast::OpKind::LessEqual,
            ">" => ast::OpKind::Greater,
            ">=" => ast::OpKind::GreaterEqual,
            _ => panic!("Expected <, <=, >, or >=."),
        },
    );

    let op_add_parser = (op_kind_parser, add_parser);
    let (rest, first_add) = add_parser(s)?;

    let (rest, op_add_vec) = multi::many0(op_add_parser).parse(rest)?;
    Ok((
        rest,
        op_add_vec
            .iter()
            .fold(first_add, |acc, (op_kind, primary)| {
                ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
                    op_kind.clone(),
                    acc,
                    primary.clone(),
                )))
            }),
    ))
}

pub fn add_parser(s: &str) -> IResult<&str, ast::Expr> {
    let op_kind_parser = combinator::map(branch::alt((tag("+"), tag("-"))), |op| match op {
        "+" => ast::OpKind::Add,
        "-" => ast::OpKind::Sub,
        _ => panic!("Expected + or -."),
    });

    let op_mul_parser = (op_kind_parser, mul_parser);
    let (rest, first_mul) = mul_parser(s)?;

    let (rest, op_mul_vec) = multi::many0(op_mul_parser).parse(rest)?;

    Ok((
        rest,
        op_mul_vec
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

    let op_unary_parser = (op_kind_parser, unary_parser);
    let (rest, first_unary) = unary_parser(s)?;

    let (rest, op_unary_vec) = multi::many0(op_unary_parser).parse(rest)?;

    Ok((
        rest,
        op_unary_vec
            .iter()
            .fold(first_unary, |acc, (op_kind, primary)| {
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

pub fn unary_parser(s: &str) -> IResult<&str, ast::Expr> {
    let (rest, minus) = combinator::opt(tag("-")).parse(s)?;
    let (rest, primary) = primary_parser(rest)?;

    match minus {
        None => Ok((rest, primary)),
        Some(_) => Ok((
            rest,
            ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
                ast::OpKind::Sub,
                ast::Expr::ConstInt(ast::ConstInt::new(0)),
                primary,
            ))),
        )),
    }
}

#[test]
pub fn unary_parser_test() {
    let (_, actual) = unary_parser("-96").unwrap();
    let expect = ast::Expr::BinaryOp(Box::new(ast::BinaryOp::new(
        ast::OpKind::Sub,
        ast::Expr::ConstInt(ast::ConstInt::new(0)),
        ast::Expr::ConstInt(ast::ConstInt::new(96)),
    )));
    assert_eq!(actual, expect);
}
