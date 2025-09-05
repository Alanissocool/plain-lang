use crate::ast::*;
use crate::runtime::Runtime;

pub fn interpret(stmt: &Stmt) -> Result<(), String> {
    let mut runtime = Runtime::new();
    runtime.exec_stmt(stmt)
}

// TODO: Implement Cranelift JIT compilation later
