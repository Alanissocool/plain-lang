use std::collections::HashMap;
use crate::ast::*;

pub struct Runtime {
    vars: HashMap<String, Value>,
    last_value: Option<Value>,
}

impl Runtime {
    pub fn new() -> Self {
        Self { vars: HashMap::new(), last_value: None }
    }

    pub fn eval_expr(&self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Int(i) => Ok(Value::Int(*i)),
            Expr::Str(s) => Ok(Value::Str(s.clone())),
            Expr::Var(name) => self.vars.get(name).cloned().ok_or_else(|| format!("I don't know the value of '{}'. Did you mean 'Set {} to ...' first?", name, name)),
            Expr::LastValue => self.last_value.clone().ok_or_else(|| "No previous result to refer to with 'it'".to_string()),
            Expr::Gt(a, b) => {
                let a = self.eval_expr(a)?;
                let b = self.eval_expr(b)?;
                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(if a > b { 1 } else { 0 })),
                    _ => Err("Type error: expected numbers for comparison".to_string()),
                }
            }
            Expr::Lt(a, b) => {
                let a = self.eval_expr(a)?;
                let b = self.eval_expr(b)?;
                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(if a < b { 1 } else { 0 })),
                    _ => Err("Type error: expected numbers for comparison".to_string()),
                }
            }
            Expr::Eq(a, b) => {
                let a = self.eval_expr(a)?;
                let b = self.eval_expr(b)?;
                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => Ok(Value::Int(if a == b { 1 } else { 0 })),
                    (Value::Str(a), Value::Str(b)) => Ok(Value::Int(if a == b { 1 } else { 0 })),
                    _ => Err("Type error: expected same types for equality".to_string()),
                }
            }
        }
    }

    pub fn exec_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Set(name, expr) => {
                let val = self.eval_expr(expr)?;
                self.vars.insert(name.clone(), val.clone());
                self.last_value = Some(val);
                Ok(())
            }
            Stmt::Add(expr, name) => {
                let val = self.eval_expr(expr)?;
                if let Some(Value::Int(current)) = self.vars.get(name) {
                    if let Value::Int(add) = val {
                        let new_val = Value::Int(current + add);
                        self.vars.insert(name.clone(), new_val.clone());
                        self.last_value = Some(new_val);
                        Ok(())
                    } else {
                        Err("Type error: expected number to add".to_string())
                    }
                } else {
                    Err(format!("I don't know the value of '{}'", name))
                }
            }
            Stmt::Sub(expr, name) => {
                let val = self.eval_expr(expr)?;
                if let Some(Value::Int(current)) = self.vars.get(name) {
                    if let Value::Int(sub) = val {
                        self.vars.insert(name.clone(), Value::Int(current - sub));
                        Ok(())
                    } else {
                        Err("Type error: expected number to subtract".to_string())
                    }
                } else {
                    Err(format!("I don't know the value of '{}'", name))
                }
            }
            Stmt::Multiply(name, expr) => {
                let val = self.eval_expr(expr)?;
                if let Some(Value::Int(current)) = self.vars.get(name) {
                    if let Value::Int(sub) = val {
                        self.vars.insert(name.clone(), Value::Int(current * sub));
                        Ok(())
                    } else {
                        Err("Type error: expected number to subtract".to_string())
                    }
                } else {
                    Err(format!("I don't know the value of '{}'", name))
                }
            }
            Stmt::Show(expr) => {
                let val = self.eval_expr(expr)?;
                match val {
                    Value::Int(i) => println!("{}", i),
                    Value::Str(s) => println!("{}", s),
                }
                Ok(())
            }
            Stmt::If(cond, then) => {
                let cond_val = self.eval_expr(cond)?;
                if let Value::Int(i) = cond_val {
                    if i != 0 {
                        self.exec_stmt(then)?;
                    }
                } else {
                    return Err("Type error: condition must be number".to_string());
                }
                Ok(())
            }
            Stmt::Seq(a, b) => {
                self.exec_stmt(a)?;
                self.exec_stmt(b)
            }
            Stmt::Loop(count, body) => {
                let count_val = self.eval_expr(count)?;
                if let Value::Int(c) = count_val {
                    for _ in 0..c {
                        self.exec_stmt(body)?;
                    }
                    Ok(())
                } else {
                    Err("Type error: loop count must be number".to_string())
                }
            }
        }
    }
}
