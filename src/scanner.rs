use std::fmt;

use super::error::*;

#[derive(Debug)]
enum TokenType {                                   
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star, 

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier, String, Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
     
    Print, Return, Super, This, True, Var, While,    

    Eof
}

#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    fn new(_type: TokenType, lexeme: String, line: usize) -> Self {
        Token { _type, lexeme, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Token({:?}): '{}' ", self._type, self.lexeme)
    }
}

pub struct Scanner {
    source: Vec<String>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let source_chars = source.chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();

        Scanner {
            source: source_chars,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_end(&self) -> bool {
        return self.current >= self.source.len();
    }

    fn advance(&mut self) -> &String {
        self.current += 1;
        return &self.source[self.current - 1];
    }

    fn next_matches(&mut self, expected: &str) -> bool {
        if self.is_end() {
            return false;
        }

        let next = &self.source[self.current];
        let matches = next == expected;

        if matches {
            self.current += 1;
            return true;
        } else {
            return false;
        }
    }

    fn peek(&self) -> String {
        match self.is_end() {
            true => String::from("\0"),
            false => self.source[self.current].clone()
        }
    }

    fn read_string_literal(&mut self) -> LoxResult<()> {
        while !self.is_end() && self.peek() != "\"" {
            if self.peek() == "\n" {
                self.line += 1;
            }

            self.advance();
        }

        // Unterminated string
        if self.is_end() {
            return Err(
                LoxError::syntax(
                    self.line,
                    format!("Unterminated string at line {}", self.line)
                )
            );
        }

        self.advance();

        // let value = String::copy();
        // let value = &self.source[self.start + 1 .. self.current - 1];

        Ok(self.add_token(TokenType::String))
    }

    fn add_token(&mut self, token_type: TokenType) {
        let start = self.start;
        let current = self.current;
        let line = self.line;
        let lexeme = self.source[start..current].to_vec().join("");

        self.tokens.push(Token::new(token_type, lexeme, line));
    }

    fn check_and_add_token(&mut self, check: &str, matches: TokenType, not_matches: TokenType) {
        let token_type = match self.next_matches(check) {
            true => matches,
            false => not_matches,
        };

        self.add_token(token_type);
    }

    pub fn scan_tokens(&mut self) -> LoxResult<&Vec<Token>> {
        loop {
            let c = self.advance();
        
            match c.as_str() {
                // General
                "(" => self.add_token(TokenType::LeftParen),
                ")" => self.add_token(TokenType::RightParen),
                "{" => self.add_token(TokenType::LeftBrace),
                "}" => self.add_token(TokenType::RightBrace),
                "," => self.add_token(TokenType::Comma),
                "." => self.add_token(TokenType::Dot),
                "-" => self.add_token(TokenType::Minus),
                "+" => self.add_token(TokenType::Plus),
                ";" => self.add_token(TokenType::Semicolon),
                "*" => self.add_token(TokenType::Star),
                "!" => self.check_and_add_token("=", TokenType::BangEqual, TokenType::Bang),
                "=" => self.check_and_add_token("=", TokenType::EqualEqual, TokenType::Equal),
                "<" => self.check_and_add_token("=", TokenType::LessEqual, TokenType::Less),
                ">" => self.check_and_add_token("=", TokenType::GreaterEqual, TokenType::Greater),

                // Comments
                "/" => {
                    if self.next_matches("/") {
                        while self.peek() != "\n" && !self.is_end() {
                            self.advance();
                        }
                    } else {
                       self.add_token(TokenType::Slash);
                    }
                },

                // Ignore
                " " => (),
                "\r" => (),
                "\t" => (),
                "\n" => {
                    self.line += 1;
                },

                // String literal
                "\"" => self.read_string_literal()?,

                // Unexpected char
                _ => {
                    return Err(
                        LoxError::syntax(
                            10,
                            format!("Unexpected character `{}` at line {}.", c.as_str(), 10).to_string()
                        )
                    );
                }
            };

            if self.is_end() {
                break;
            }
        }

        return Ok(&self.tokens);
    }
}
