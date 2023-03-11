use std::collections::HashMap;

use crate::tokens::{Token, TokenKind};

pub struct Parser {
    pub tokens: Vec<Token>,
}

impl Parser {
    pub fn parse(&self) -> HashMap<String, String> {
        let mut result: HashMap<String, String> = HashMap::new();
        let mut current_key: Option<String> = None;
        let tokens_iter = self.tokens.iter();

        for token in tokens_iter {
            match &current_key {
                Some(key) => match token.kind {
                    TokenKind::Key => continue,
                    TokenKind::EqSign => continue,
                    TokenKind::Value => match &token.value {
                        Some(value) => {
                            result.insert(key.to_string(), value.to_string());
                            current_key = None;
                            continue;
                        }
                        None => continue,
                    },
                },
                None => match token.kind {
                    TokenKind::Key => match &token.value {
                        Some(value) => current_key = Some(value.to_string()),
                        None => continue,
                    },
                    TokenKind::EqSign => continue,
                    TokenKind::Value => continue,
                },
            }
        }

        return result;
    }
}
