struct CodeBuffer<T> {
    value: T,
    size: usize
}

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Identifier(String),
    Equals, Minus, Plus, Star, Slash, Hat,
    LeftParenthesis, RightParenthesis
}

#[derive(Debug)]
pub struct TokenCollection {
    tokens: Vec<Token>
}

impl TokenCollection {
    pub fn head(self: &TokenCollection) -> Option<&Token> {
        self.get(0)
    }

    pub fn get(self: &TokenCollection, i: usize) -> Option<&Token> {
        match self.tokens.get(i) {
            Some(token) => Some(&token),
            None => None
        }
    }

    pub fn len(self: &TokenCollection) -> usize {
        self.tokens.len()
    }

    pub fn tokens(self: &TokenCollection) -> &[Token] {
        &self.tokens
    }
}

pub fn lex(code: &str) -> Result<TokenCollection, &'static str> {
    let symbols: Vec<char> = code.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut start: usize = 0;

    loop {
        let mut offset = 1;

        let some_token = match symbols.get(start) {
            Some(' ') => None,
            Some('=') => Some(Token::Equals),
            Some('-') => Some(Token::Minus),
            Some('+') => Some(Token::Plus),
            Some('*') => Some(Token::Star),
            Some('/') => Some(Token::Slash),
            Some('^') => Some(Token::Hat),
            Some('(') => Some(Token::LeftParenthesis),
            Some(')') => Some(Token::RightParenthesis),
            Some(ch) => {
                if ch.is_digit(10) {
                    let buff = read_number(&symbols[start..]);

                    offset = buff.size;

                    Some(Token::Number(buff.value))
                } else if ch.is_alphabetic() || *ch == '_' {
                    let buff = read_identifier(&symbols[start..]);

                    offset = buff.size;

                    Some(Token::Identifier(buff.value))
                } else {
                    return Err("There is an invalid character in your expression")
                }
            },
            None => break
        };

        match some_token {
            Some(token) => tokens.push(token),
            None => {}
        };

        start += offset;
    }

    Ok(TokenCollection { tokens })
}

fn read_number(code: &[char]) -> CodeBuffer<f64> {
    let (multiplier, starting_index): (f64, usize) = match code.get(0) {
        Some('-') => (-1.0, 1),
        _ => (1.0, 0)
    };

    let mut num: f64 = 0.0;
    let mut consumed_chars: usize = starting_index;
    let mut reading_decimal: bool = false;
    let mut decimal_size: f64 = 0.0;

    for c in code[starting_index..].iter() {
        if *c == '.' {
            reading_decimal = true;

            consumed_chars += 1;
            continue;
        }

        match c.to_digit(10) {
            Some(digit) => num = num * 10_f64 + (digit as f64),
            None        => break,
        };

        if reading_decimal {
            decimal_size += 1.0;
        }

        consumed_chars += 1;
    }

    return CodeBuffer { value: num * multiplier / 10_f64.powf(decimal_size), size: consumed_chars };
}

fn read_identifier(code: &[char]) -> CodeBuffer<String> {
    let mut identifier = String::from("");
    let mut consumed_chars: usize = 0;

    for c in code.iter() {
        if !c.is_alphabetic() && *c != '_' {
            break;
        }

        identifier.push(*c);
        consumed_chars += 1;
    }

    return CodeBuffer { value: identifier, size: consumed_chars };
}