/// 式を表す。
#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    ConstInt(ConstInt),
    BinaryOp(Box<BinaryOp>),
    Assign(Box<Assign>),
    LocalVar(Box<LocalVar>),
}

impl Node {
    /// 式に対応するアセンブリを生成する。
    pub fn generate(&self) {
        match self {
            Node::ConstInt(e) => e.generate(),
            Node::BinaryOp(e) => e.generate(),
            Node::Assign(e) => e.generate(),
            Node::LocalVar(e) => e.generate(),
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
    left_node: Node,
    right_node: Node,
}

impl BinaryOp {
    pub fn new(op_kind: OpKind, left_node: Node, right_node: Node) -> BinaryOp {
        BinaryOp {
            op_kind,
            left_node,
            right_node,
        }
    }

    /// 2項演算の評価。
    pub fn generate(&self) {
        self.left_node.generate();
        self.right_node.generate();

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

#[derive(Debug, PartialEq, Clone)]
pub struct Assign {
    lhs: Node,
    rhs: Option<Node>,
}

impl Assign {
    pub fn new(lhs: Node, rhs: Option<Node>) -> Assign {
        Assign { lhs, rhs }
    }

    pub fn generate(&self) {
        match &self.rhs {
            Some(rhs) => match &self.lhs {
                Node::LocalVar(lhs) => {
                    lhs.generate_address();
                    rhs.generate();
                    println!("  pop rdi");
                    println!("  pop rax");
                    println!("  mov [rax], rdi");
                    println!("  push rdi");
                }
                _ => panic!("Lhs is not a variable."),
            },
            None => self.lhs.generate(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LocalVar {
    offset: i32,
}

impl LocalVar {
    pub fn new(offset: i32) -> LocalVar {
        LocalVar { offset }
    }

    pub fn generate_address(&self) {
        println!("  mov rax, rsp");
        println!("  sub rax, {}", self.offset);
        println!("  push rax");
    }

    pub fn generate(&self) {
        println!("  mov rax, rsp");
        println!("  sub rax, {}", self.offset);
        println!("  mov rax, [rax]");
        println!("  push rax");
    }
}
