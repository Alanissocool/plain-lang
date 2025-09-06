use crate::ast::*;
use crate::lexer::Token;

pub fn parse(input: &str) -> Result<Stmt, String> {
    let tokens = crate::lexer::lex(input);
    if tokens.is_empty() {
        return Err("Empty line".to_string());
    }
    let mut pos = 0;
    let stmt = parse_stmt(&tokens, &mut pos)?;
    if pos < tokens.len() {
        if matches!(tokens[pos], Token::Period) {
            pos += 1;
        }
        if pos < tokens.len() {
            return Err("Extra tokens".to_string());
        }
    }
    Ok(stmt)
}

fn parse_stmt(tokens: &[Token], pos: &mut usize) -> Result<Stmt, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of tokens".to_string());
    }
    let mut stmt = match &tokens[*pos] {
        Token::Set => {
            *pos += 1;
            if matches!(tokens[*pos], Token::The) {
                *pos += 1;
            }
            if let Token::Ident(name) = &tokens[*pos] {
                *pos += 1;
                if matches!(tokens[*pos], Token::To) {
                    *pos += 1;
                    let expr = parse_expr(tokens, pos)?;
                    Stmt::Set(name.clone(), expr)
                } else {
                    return Err("Expected 'to'".to_string());
                }
            } else {
                return Err("Expected identifier".to_string());
            }
        }
        Token::Add => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            if matches!(tokens[*pos], Token::To) {
                *pos += 1;
                if matches!(tokens[*pos], Token::The) {
                    *pos += 1;
                }
                if let Token::Ident(name) = &tokens[*pos] {
                    *pos += 1;
                    Stmt::Add(expr, name.clone())
                } else {
                    return Err("Expected identifier".to_string());
                }
            } else {
                return Err("Expected 'to'".to_string());
            }
        }
        Token::Sub => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            if matches!(tokens[*pos], Token::From) {
                *pos += 1;
                if matches!(tokens[*pos], Token::The) {
                    *pos += 1;
                }
                if let Token::Ident(name) = &tokens[*pos] {
                    *pos += 1;
                    Stmt::Sub(expr, name.clone())
                } else {
                    return Err("Expected identifier".to_string());
                }
            } else {
                return Err("Expected 'from'".to_string());
            }
        }
        Token::Multiply => {
            *pos += 1;            
            if matches!(tokens[*pos], Token::The) {
                *pos += 1;
            }
            if let Token::Ident(name) = &tokens[*pos] {
                *pos += 1;
                if matches!(tokens[*pos], Token::By) {
                    *pos += 1;
                    let expr = parse_expr(tokens, pos)?;
                    Stmt::Multiply(name.clone(), expr)
                } else {
                    return Err("Expected 'by'".to_string());
                }
            } else {
                return Err("Expected identifier".to_string());
            }
            
        }
        Token::Show => {
            *pos += 1;
            if matches!(tokens[*pos], Token::On) {
                *pos += 1;
                if matches!(tokens[*pos], Token::The) {
                    *pos += 1;
                }
                if matches!(tokens[*pos], Token::Screen) {
                    *pos += 1;
                } else {
                    return Err("Expected 'screen'".to_string());
                }
            }
            if matches!(tokens[*pos], Token::The) {
                *pos += 1;
            }
            let expr = parse_expr(tokens, pos)?;
            Stmt::Show(expr)
        }
        Token::Print => {
            *pos += 1;
            if matches!(tokens[*pos], Token::The) {
                *pos += 1;
            }
            let expr = parse_expr(tokens, pos)?;
            Stmt::Show(expr)
        }
        Token::If => {
            *pos += 1;
            if matches!(tokens[*pos], Token::The) {
                *pos += 1;
            }
            let expr = parse_expr(tokens, pos)?;
            if matches!(tokens[*pos], Token::Then) {
                *pos += 1;
                let stmt = parse_stmt(tokens, pos)?;
                Stmt::If(expr, Box::new(stmt))
            } else {
                return Err("Expected 'then'".to_string());
            }
        }
        Token::Count => {
            *pos += 1;
            if matches!(tokens[*pos], Token::To) {
                *pos += 1;
                let count = parse_expr(tokens, pos)?;
                // Skip "and when you are done"
                while *pos < tokens.len() && !matches!(tokens[*pos], Token::Display) && !matches!(tokens[*pos], Token::Show) {
                    *pos += 1;
                }
                let body = parse_stmt(tokens, pos)?;
                Stmt::Loop(count, Box::new(body))
            } else {
                return Err("Expected 'to'".to_string());
            }
        }
        Token::Display => {
            *pos += 1;
            if matches!(tokens[*pos], Token::The) {
                *pos += 1;
            }
            let expr = parse_expr(tokens, pos)?;
            if matches!(tokens[*pos], Token::Result) {
                *pos += 1;
            }
            Stmt::Show(expr)
        }
        _ => return Err("Unknown statement".to_string()),
    };

    // Check for "then" for sequences
    while *pos < tokens.len() && matches!(tokens[*pos], Token::Then) {
        *pos += 1;
        if *pos >= tokens.len() {
            return Err("Expected statement after 'then'".to_string());
        }
        let next = parse_stmt(tokens, pos)?;
        stmt = Stmt::Seq(Box::new(stmt), Box::new(next));
    }

    Ok(stmt)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of tokens".to_string());
    }
    let left = parse_atom(tokens, pos)?;
    if *pos < tokens.len() && matches!(tokens[*pos], Token::Is) {
        *pos += 1;
        if matches!(tokens[*pos], Token::Greater) {
            *pos += 1;
            if matches!(tokens[*pos], Token::Than) {
                *pos += 1;
                let right = parse_expr(tokens, pos)?;
                Ok(Expr::Gt(Box::new(left), Box::new(right)))
            } else {
                Err("Expected 'than'".to_string())
            }
        } else if matches!(tokens[*pos], Token::Less) {
            *pos += 1;
            if matches!(tokens[*pos], Token::Than) {
                *pos += 1;
                let right = parse_expr(tokens, pos)?;
                Ok(Expr::Lt(Box::new(left), Box::new(right)))
            } else {
                Err("Expected 'than'".to_string())
            }
        } else if matches!(tokens[*pos], Token::Equal) {
            *pos += 1;
            if matches!(tokens[*pos], Token::To) {
                *pos += 1;
                let right = parse_expr(tokens, pos)?;
                Ok(Expr::Eq(Box::new(left), Box::new(right)))
            } else {
                Err("Expected 'to'".to_string())
            }
        } else {
            Err("Expected comparison".to_string())
        }
    } else {
        Ok(left)
    }
}

fn parse_atom(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of tokens".to_string());
    }
    if matches!(tokens[*pos], Token::The) {
        *pos += 1;
    }
    if *pos >= tokens.len() {
        return Err("Unexpected end of tokens".to_string());
    }
    match &tokens[*pos] {
        Token::Int(s) => {
            *pos += 1;
            Ok(Expr::Int(s.parse().unwrap()))
        }
        Token::Str(s) => {
            *pos += 1;
            Ok(Expr::Str(s[1..s.len() - 1].to_string()))
        }
        Token::Ident(s) => {
            *pos += 1;
            // Handle "it" as pronoun
            if s == "it" {
                Ok(Expr::LastValue)
            } else {
                Ok(Expr::Var(s.clone()))
            }
        }
        _ => Err("Unknown atom".to_string()),
    }
}
