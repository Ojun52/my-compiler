/// 式を表す。
pub enum Expr {
    ConstInt(ConstInt),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    /// 式に対応するアセンブリを生成する。
    pub fn generate(&self) -> i32 {
        match self {
            Expr::ConstInt(e) => e.generate(),
            Expr::BinaryOp(e) => e.generate(),
        }
    }
}

// タプルで定義されている。
/// 32bit整数定数
pub struct ConstInt(i32);

impl CosntInt {
    /// ConstIntを生成。
    pub fn new(value: i32) -> ConstInt {
        ConstInt(value)
    }

    /// 評価関数。
    pub fn generate(&self) {
        println!("  push {}", self.0);
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
    pub fn generate(&self) {
        let lhs = self.left_expr.generate();
        let rhs = self.right_expr.generate();

        println!("  pop rdi");
        println!("  pop rax");

        match self.op_kind {
            OpKind::Add => println!("  add rax, rdi"),
            OpKind::Sub => println!("  sub rax, rdi"),
            OpKind::Mul => println!("  mul rax, rdi"),
            OpKind::Div => {
                println!("  cqo");
                println!("  idiv rdi")
            }
        }

        println!("  push rax");
    }
}
