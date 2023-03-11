#[derive(Debug)]
pub enum TokenKind {
    Key,
    EqSign,
    Value,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: Option<String>,
}

impl Token {
    pub fn new_key(name: String) -> Token {
        return Token {
            kind: TokenKind::Key,
            value: Some(name),
        };
    }

    pub fn new_value(value: String) -> Token {
        return Token {
            kind: TokenKind::Value,
            value: Some(value),
        };
    }

    pub fn new_eq() -> Token {
        return Token {
            kind: TokenKind::EqSign,
            value: None,
        };
    }
}
