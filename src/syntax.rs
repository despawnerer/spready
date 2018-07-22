use reference::Reference;

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Reference(Reference),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
