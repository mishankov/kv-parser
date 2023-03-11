use crate::tokens::Token;

#[derive(Debug)]
pub struct Lexer {
    text: String,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let lexer = Lexer { text };
        return lexer;
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        let lines = self.text.lines();

        for line in lines {
            let mut kv = line.split("=");
            let key = kv.next();
            let value = kv.next();

            match key {
                Some(key) => tokens.push(Token::new_key(String::from(key))),
                None => todo!(),
            }

            tokens.push(Token::new_eq());

            match value {
                Some(value) => tokens.push(Token::new_value(String::from(value))),
                None => todo!(),
            }
        }

        return tokens;
    }
}
