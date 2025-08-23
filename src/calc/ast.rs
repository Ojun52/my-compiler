/// 式を表す。
pub enum Expr {
    ConstInt(ConstInt),
    PlusOp(Box<BinaryOp>),
}

impl Expr {
    /// 式を評価する。
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstInt(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval(),
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

pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
}

/// 基本的な2項演算。
pub struct BinaryOp {
    op_kind: OpKind,
    left_expr: Expr,
    right_expr: Expr,
}

impl BinaryOp {
    pub fn new(op_kind: OpKind, left_expr: Expr, right_expr: Expr) -> BinaryOp {
        BinaryOp {
            op_kind,
            left_expr,
            right_expr,
        }
    }

    /// 2項演算の評価。
    pub fn eval(&self) {
        let lhs = self.left_expr.eval();
        let rhs = self.right_expr.eval();

        match self.op_kind {
            OpKind::Add => left + right,
            OpKind::Sub => left - right,
            OpKind::Mul => left * right,
            OpKind::Div => left / right,
        }
    }
}

#[cfg[test]]
mod tests {
    #[test]
    fn binary_op_test() {
        /// 5*(2+3)
        let binary_op = BinaryOp::new(
            OpKind::Mul,
            Expr::ConstInt(ConstInt::new(5)),
            Expr::BinaryOp(Box::new(BinaryOp::new(
                OpKind::Add,
                Expr::ConstInt::new(2),
                Expr::ConstInt::new(3),
            ))),
        );

        let expect = 5 + (2 + 3);
        assert_eq!(binary_op.eval(), expect);
    }

    #[test]
    fn constint_test() {
        let expect = 55;
        let const_int = ConstInt::new(expect);
        assert_eq!(const_int.eval(), expect);
    }
}
