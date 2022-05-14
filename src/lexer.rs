use std::{iter::Peekable, str::Chars};

use phf::phf_map;

use crate::token::Token;

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

static SYMBOL_MAP: phf::Map<char, Token> = phf_map! {
    '=' => Token::ASSIGN,
    '+' => Token::PLUS,
    '-' => Token::MINUS,
    '!' => Token::BANG,
    '*' => Token::ASTERISK,
    '/' => Token::SLASH,
    '<' => Token::LT,
    '>' => Token::GT,
    ',' => Token::COMA,
    ';' => Token::SEMICOLON,
    '(' => Token::LPAREN,
    ')' => Token::RPAREN,
    '{' => Token::LBRACE,
    '}' => Token::RBRACE,
};

static KEYWORD_MAP: phf::Map<&'static str, Token> = phf_map! {
    "let" => Token::LET,
    "fn" => Token::FUNCTION,
    "true" => Token::TRUE,
    "false" => Token::FALSE,
};

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input: input.chars().peekable(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.consume_whitespace();
        self._next_token()
    }

    fn _next_token(&mut self) -> Token {
        match self.peek() {
            Some('=') => {
                let _ = self.consume();
                if let Some('=') = self.peek() {
                    let _ = self.consume();
                    Token::EQUALS
                } else {
                    Token::ASSIGN
                }
            }
            Some('!') => {
                let _ = self.consume();
                if let Some('=') = self.peek() {
                    let _ = self.consume();
                    Token::NOTEQUALS
                } else {
                    Token::BANG
                }
            }
            Some(ch) => match SYMBOL_MAP.get(ch) {
                Some(token) => {
                    let _ = self.consume();
                    token.clone()
                }
                None => {
                    if is_letter(ch) {
                        let identifier = self.consume_ident();
                        match KEYWORD_MAP.get(&identifier) {
                            Some(token) => token.clone(),
                            None => Token::IDENT(identifier),
                        }
                    } else if is_number(ch) {
                        Token::INT(self.consume_int())
                    } else {
                        Token::ILLEGAL
                    }
                }
            },
            None => Token::EOF,
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn consume(&mut self) -> Option<char> {
        self.input.next()
    }

    fn consume_whitespace(&mut self) {
        while let Some(ch) = self.input.peek() {
            if is_whitespace(ch) {
                self.input.next();
            } else {
                break;
            }
        }
    }

    fn consume_ident(&mut self) -> String {
        let mut identifier = String::new();

        while is_letter(self.input.peek().unwrap()) {
            identifier.push(self.input.next().unwrap());
        }

        identifier
    }

    fn consume_int(&mut self) -> u32 {
        let mut int = String::new();

        while is_number(self.input.peek().unwrap()) {
            int.push(self.input.next().unwrap());
        }

        int.parse::<u32>().unwrap()
    }
}

fn is_letter(ch: &char) -> bool {
    ch.is_ascii_alphabetic() || *ch == '_'
}

fn is_number(ch: &char) -> bool {
    ch.is_ascii_digit()
}

fn is_whitespace(ch: &char) -> bool {
    ch.is_ascii_whitespace()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    #[test]
    fn next_token_works() {
        let input = indoc! {"
            let five = 5;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);

            let sum = 2 - 3 * 4 + 4/2;

            false = !true;

            1 < 2 > 3;

            10 == 10;
            10 != 9;
        "};

        let mut lexer = Lexer::new(input);

        let tokens = [
            Token::LET,
            Token::IDENT("five".to_string()),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("ten".to_string()),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("add".to_string()),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT("x".to_string()),
            Token::COMA,
            Token::IDENT("y".to_string()),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT("x".to_string()),
            Token::PLUS,
            Token::IDENT("y".to_string()),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("result".to_string()),
            Token::ASSIGN,
            Token::IDENT("add".to_string()),
            Token::LPAREN,
            Token::IDENT("five".to_string()),
            Token::COMA,
            Token::IDENT("ten".to_string()),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT("sum".to_string()),
            Token::ASSIGN,
            Token::INT(2),
            Token::MINUS,
            Token::INT(3),
            Token::ASTERISK,
            Token::INT(4),
            Token::PLUS,
            Token::INT(4),
            Token::SLASH,
            Token::INT(2),
            Token::SEMICOLON,
            Token::FALSE,
            Token::ASSIGN,
            Token::BANG,
            Token::TRUE,
            Token::SEMICOLON,
            Token::INT(1),
            Token::LT,
            Token::INT(2),
            Token::GT,
            Token::INT(3),
            Token::SEMICOLON,
            Token::INT(10),
            Token::EQUALS,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NOTEQUALS,
            Token::INT(9),
            Token::SEMICOLON,
            Token::EOF,
        ];

        for token in tokens.into_iter() {
            assert_eq!(token, lexer.next_token());
        }
    }
}
