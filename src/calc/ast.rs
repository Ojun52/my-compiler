/// 式を表す。
pub enum Expr {
    ConstInt(ConstInt),
    PlusOp(Box<PlusOp>),
}

impl Expr {
    /// 式を評価する。
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstInt(e) => e.eval(),
            Expr::PlusOp(e) => e.eval(),
        }
    }
}

// タプルで定義されている。
/// 32bit整数定数
pub struct ConstInt(i32);

impl CosntInt {
    /// ConstIntを生成。
    pub fn new(value: i32) -> ConstInt {
        ConstInt(value);
    }

    /// 評価関数。
    pub fn eval(&self) -> i32 {
        self.0
    }
}

#[test]
fn constint_test() {
    let expect = 55;
    let const_int = ConstInt::new(expect);
    assert_eq!(const_int.eval(), expect);
}

/// 加法を表す。
pub struct PlusOp {
    left_expr: Expr,
    right_expr: Expr,
}

impl PlusOp {
    pub fn new(left_expr: Expr, right_expr: Expr) -> PlusOp {
        PlusOp {
            left_expr,
            right_expr,
        }
    }

    /// 加法の評価。
    pub fn eval(&self) -> i32 {
        self.left_expr.eval() + self.right_expr.eval()
    }
}

#[test]
fn plus_op_test() {
    /// 1+(2+3)
    let plus_op = PlusOp::new(
        Expr::ConstInt(1),
        Expr::PlusOp::new(Box::new(PlusOp::new(
            Expr::ConstInt::new(2),
            Expr::ConstInt::new(3),
        ))),
    );

    let expect = 1 + (2 + 3);
    assert_eq!(plus_op.eval(), expect);
}
