/// 式を表す。
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    ConstInt(ConstInt),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    /// 式に対応するアセンブリを生成する。
    pub fn generate(&self) {
        match self {
            Expr::ConstInt(e) => e.generate(),
            Expr::BinaryOp(e) => e.generate(),
        }
    }
}

// タプルで定義されている。
/// 32bit整数定数
#[derive(Debug, PartialEq, Clone)]
pub struct ConstInt(i32);

impl ConstInt {
    /// ConstIntを生成。
    pub fn new(value: i32) -> ConstInt {
        ConstInt(value)
    }

    /// 評価関数。
    pub fn generate(&self) {
        println!("  push {}", self.0);
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

/// 基本的な2項演算。
#[derive(Debug, PartialEq, Clone)]
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
        self.left_expr.generate();
        self.right_expr.generate();

        println!("  pop rdi");
        println!("  pop rax");

        match self.op_kind {
            OpKind::Add => println!("  add rax, rdi"),
            OpKind::Sub => println!("  sub rax, rdi"),
            OpKind::Mul => println!("  imul rax, rdi"),
            OpKind::Div => {
                println!("  cqo");
                println!("  idiv rdi")
            }
            OpKind::Equal => {
                println!("  cmp rax, rdi");
                println!("  sete al");
                println!("  movzb rax, al");
            }
            OpKind::NotEqual => {
                println!("  cmp rax, rdi");
                println!("  setne al");
                println!("  movzb rax, al");
            }
            OpKind::Less => {
                println!("  cmp rax, rdi");
                println!("  setl al");
                println!("  movzb rax, al");
            }
            OpKind::LessEqual => {
                println!("  cmp rax, rdi");
                println!("  setle al");
                println!("  movzb rax, al");
            }
            OpKind::Greater => {
                println!("  cmp rax, rdi");
                println!("  setg al");
                println!("  movzb rax, al");
            }
            OpKind::GreaterEqual => {
                println!("  cmp rax, rdi");
                println!("  setge al");
                println!("  movzb rax, al");
            }
        }

        println!("  push rax");
    }
}
