use crate::lexer;
use crate::lexer::Token;
use crate::lexer::TokenCollection;

pub enum Stmt {
	Attr(Token, Expr),
	Expr(Expr)
}

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Literal(Token),
    Grouping(Box<Expr>)
}

pub fn parse(tokens: &TokenCollection) -> Result<Stmt, &'static str> {
	match tokens.head() {
		Some(Token::Identifier(_)) => attr_stmt(tokens.tokens()),
		Some(Token::Number(_))     => expr_stmt(tokens.tokens()),
		None => Ok(Stmt::Expr(Expr::Binary(Box::new(Expr::Literal(Token::Number(0))), Token::Plus, Box::new(Expr::Literal(Token::Number(0)))))),
		_ => Err("Sorry, this doesn't appear to be correct")
	}
}

fn attr_stmt(tokens: &[Token]) -> Result<Stmt, &'static str> {
	let identifier       = tokens.get(0);
	let equal_sign       = tokens.get(1);
	let remaining_tokens = &tokens[2..];

	let identifier = match identifier {
		Some(Token::Identifier(id)) => Token::Identifier(id.clone()),
		_ => return Err("Expected an identifier"),
	};

	match equal_sign {
		Some(Token::Equals) => (),
		_ => return Err("Expected an '=' sign"),
	};

	if remaining_tokens.len() == 0 {
		return Err("Expected expression after '='");
	}

	Ok(Stmt::Attr(identifier, expr(remaining_tokens).0))
}

fn expr_stmt(tokens: &[Token]) -> Result<Stmt, &'static str> {
	Ok(Stmt::Expr(expr(tokens).0))
}

fn expr(tokens: &[Token]) -> (Expr, &[Token]) {
	sum(tokens)
}


fn sum(tokens: &[Token]) -> (Expr, &[Token]) {
	let (lexpr, lexpr_remaining_tokens) = mult(tokens);

	match lexpr_remaining_tokens.get(0) {
		Some(Token::Plus)  => {
			let (rexpr, rexpr_remaining_tokens) = sum(&lexpr_remaining_tokens[1..]);
			(Expr::Binary(Box::new(lexpr), Token::Plus, Box::new(rexpr)), &rexpr_remaining_tokens)
		}, 
		Some(Token::Minus) => {
			let (rexpr, rexpr_remaining_tokens) = sum(&lexpr_remaining_tokens[1..]);
			(Expr::Binary(Box::new(lexpr), Token::Minus, Box::new(rexpr)), &rexpr_remaining_tokens)
		},
		_ => (lexpr, lexpr_remaining_tokens)
	}
}

fn mult(tokens: &[Token]) -> (Expr, &[Token]) {
	let (lexpr, lexpr_remaining_tokens) = neg(tokens);

	match lexpr_remaining_tokens.get(0) {
		Some(Token::Star)  => {
			let (rexpr, rexpr_remaining_tokens) = mult(&lexpr_remaining_tokens[1..]);
			(Expr::Binary(Box::new(lexpr), Token::Star, Box::new(rexpr)), &rexpr_remaining_tokens)
		},
		Some(Token::Slash) => {
			let (rexpr, rexpr_remaining_tokens) = mult(&lexpr_remaining_tokens[1..]);
			(Expr::Binary(Box::new(lexpr), Token::Slash, Box::new(rexpr)), &rexpr_remaining_tokens)
		},
		Some(Token::Hat)   => {
			let (rexpr, rexpr_remaining_tokens) = mult(&lexpr_remaining_tokens[1..]);
			(Expr::Binary(Box::new(lexpr), Token::Hat, Box::new(rexpr)), &rexpr_remaining_tokens)
		},
		_ => (lexpr, lexpr_remaining_tokens),
	}
}

fn neg(tokens: &[Token]) -> (Expr, &[Token]) {
	match tokens.get(0) {
		Some(Token::Minus) => {
			let (expr, remaining_tokens) = literal(&tokens[1..]).unwrap();

			(Expr::Unary(Token::Minus, Box::new(expr)), &remaining_tokens)
		},
		Some(Token::Plus)  => {
			let (expr, remaining_tokens) = literal(&tokens[1..]).unwrap();

			(Expr::Unary(Token::Plus,  Box::new(expr)), &remaining_tokens)
		},
		_ => literal(tokens).unwrap(),
	}
}

fn literal(tokens: &[Token]) -> Result<(Expr, &[Token]), &'static str> {
	match tokens.get(0) {
		Some(Token::Identifier(id))  => Ok((Expr::Literal(Token::Identifier(id.clone())), &tokens[1..])),
		Some(Token::Number(num))     => Ok((Expr::Literal(Token::Number(*num)), &tokens[1..])),
		Some(Token::LeftParenthesis) => Ok(grouping(tokens).unwrap()),
		_ => Err("Invalid token encoutered in expression"),
	}
}

fn grouping(tokens: &[Token]) -> Result<(Expr, &[Token]), &'static str> {
	match tokens.get(0) {
		Some(Token::LeftParenthesis) => {},
		_ => return Err("Expected opening '('"),
	};

	let (expr, remaining_tokens) = expr(tokens);

	match remaining_tokens.get(0) {
		Some(Token::RightParenthesis) => Ok((Expr::Grouping(Box::new(expr)), &remaining_tokens[1..])),
		_ => Err("Expected closing ')'")
	}
}