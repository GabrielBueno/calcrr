use crate::lexer;
use crate::lexer::Token;
use crate::lexer::TokenCollection;

struct AttrStmt {
    identifier: String,
    expr: BinaryExpr
}

enum Expr {
    Binary(Expr, Token, Expr),
    Unary(Token, Expr),
    Literal(Token),
    Grouping(Expr)
}

pub fn parse(tokens: &TokenCollection) -> Result<Expr, &'static str> {
    match tokens.head() {

    }
}