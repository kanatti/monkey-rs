use std::{
    iter::Peekable,
    str::Chars,
};

use phf::phf_map;

use crate::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

static SYMBOL_MAP: phf::Map<char, Token> = phf_map! {
    '=' => Token::ASSIGN,
    '+' => Token::PLUS,
    ',' => Token::COMA,
    ';' => Token::SEMICOLON,
    '(' => Token::LPAREN,
    ')' => Token::RPAREN,
    '{' => Token::LBRACE,
    '}' => Token::RBRACE,
};

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.peek() {
            Some(ch) => match SYMBOL_MAP.get(ch) {
                Some(token) => {
                    let _ = self.consume();
                    token.clone()
                }
                None => Token::ILLEGAL,
            },
            None => Token::ILLEGAL,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_token_works() {
        let input = "=+(){},;";
        let mut lexer = Lexer::new(input);

        let test_cases = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMA,
            Token::SEMICOLON,
        ];

        for test_case in test_cases.into_iter() {
            assert_eq!(test_case, lexer.next_token());
        }
    }
}
