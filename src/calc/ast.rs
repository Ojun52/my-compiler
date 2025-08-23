/// 式を表す。
pub enum Expr {
    ConstInt(ConstInt),
    PlusOp(Box<PlusOp>),
}

// タプルで定義されている。
/// 32bit整数定数
pub struct ConstInt(i32);

impl CosntInt {
    /// ConstIntを生成。
    pub fn new(value: i32) -> ConstInt {
        ConstInt(value);
    }

    /// getter。
    pub fn get(&self) -> i32 {
        self.0
    }
}

#[test]
fn constint_test() {
    let expect = 55;
    let const_int = ConstInt::new(expect);
    assert_eq!(const_int.get(), expect);
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
}
