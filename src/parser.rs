use std::collections::HashMap;

use crate::tokens::{Token, TokenKind};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn parse(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        let mut current_key: Option<String> = None;

        for token in self.tokens.iter() {
            match (&token.kind, &token.value, &current_key) {
                (TokenKind::Key, Some(key), None) => current_key = Some(key.to_string()),
                (TokenKind::Value, Some(value), Some(key)) => {
                    result.insert(key.to_string(), value.to_string());
                    current_key = None;
                }
                (_, _, _) => continue,
            }
        }
        return result;
    }
}
