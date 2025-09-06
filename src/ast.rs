#[derive(Clone)]
pub enum Value {
    Int(i64),
    Str(String),
}

pub enum Expr {
    Int(i64),
    Str(String),
    Var(String),
    LastValue,
    Gt(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
}

pub enum Stmt {
    Set(String, Expr),
    Add(Expr, String),
    Sub(Expr, String),
    Multiply(String, Expr),
    Show(Expr),
    If(Expr, Box<Stmt>),
    Seq(Box<Stmt>, Box<Stmt>),
    Loop(Expr, Box<Stmt>),
}
