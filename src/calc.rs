use std::collections::HashMap;

use crate::parser::Stmt;
use crate::parser::Expr;
use crate::lexer::Token;

pub struct Calc {
    mem: HashMap<String, f64>,
}

impl Calc {
    pub fn new() -> Calc {
        Calc { mem: HashMap::new() }
    }

    pub fn run(self: &mut Calc, stmt: &Stmt) -> Result<f64, &'static str> {
        match stmt {
            Stmt::Expr(expr) => self.run_expr(expr),
            Stmt::Attr(token, expr) => {
                let resolved_expr = self.run_expr(expr);

                match resolved_expr {
                    Ok(result) => self.memadd(token, result),
                    Err(err) => Err(err)
                }
            }
        }
    }

    fn run_expr(self: &Calc, expr: &Expr) -> Result<f64, &'static str> {
        // println!("{:?}", expr);

        match expr {
            Expr::Binary(lexpr, op, rexpr) => {
                let lresult = match self.run_expr(&lexpr) {
                    Ok(val) => val,
                    Err(err) => return Err(err)
                };

                let rresult = match self.run_expr(&rexpr) {
                    Ok(val) => val,
                    Err(err) => return Err(err)
                };

                match op {
                    Token::Plus  => Ok(lresult + rresult),
                    Token::Minus => Ok(lresult - rresult),
                    Token::Star  => Ok(lresult * rresult),
                    Token::Slash => Ok(lresult / rresult),
                    Token::Hat   => Ok(lresult.powf(rresult)),
                    _ => Err("I'm afraid I can't do this")
                }
            },
            Expr::Unary(op, rexpr) => {
                let rresult = match self.run_expr(&rexpr) {
                    Ok(val) => val,
                    Err(err) => return Err(err)
                };

                match op {
                    Token::Plus  => Ok(rresult),
                    Token::Minus => Ok(-rresult),
                    _ => Err("I'm afraid I can't do this")
                }
            },
            Expr::Literal(literal) => {
                match literal {
                    Token::Number(val)    => Ok(*val),
                    Token::Identifier(id) => self.memget(&id),
                    _ => Err("There is something wrong in your expression")
                }
            },
            Expr::Grouping(expr) => {
                return self.run_expr(&expr)
            }
        }
    }

    fn memadd(self: &mut Calc, token: &Token, val: f64) -> Result<f64, &'static str> {
        match token {
            Token::Identifier(id) => {
                self.mem.insert(id.clone(), val);
                Ok(val)
            },
            _ => Err("I can't attribute a value to this identifier")
        }
    }

    fn memget(self: &Calc, identifier: &str) -> Result<f64, &'static str> {
        match self.mem.get(identifier) {
            Some(val) => Ok(*val),
            None => Err("There is something in your expression that I cannot understand")
        }
    }
}