use std::{ops::Range, str::Chars};

pub type Span = Range<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Text,
    Whitespace,
    HardLinebreak,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    AttributeIdentifier,
    FunctionIdentifier,
    ArgumentSeparator,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self { token_type, span }
    }
}

pub struct Lexer<'input> {
    chars: Chars<'input>,
    start: usize,
    current: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            chars: input.trim().chars(),
            start: 0,
            current: 0,
        }
    }

    fn token(&mut self, token_type: TokenType) -> Token {
        let span = self.start..self.current;
        self.start = self.current;
        Token::new(token_type, span)
    }

    #[inline]
    fn consume(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    #[inline]
    fn peek(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    #[inline]
    fn peek_next(&mut self) -> Option<char> {
        self.chars.clone().nth(1)
    }

    fn identifier(&mut self, token_type: TokenType) -> Token {
        let is_valid_char = |c: char| {
            c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-'
        };

        loop {
            match self.peek() {
                Some(c) if is_valid_char(c) => {
                    self.consume();
                }
                _ => break,
            }
        }

        self.token(token_type)
    }

    fn text(&mut self) -> Token {
        let is_invalid_char =
            |c: char| ['[', ']', '(', ')', '|', '#', '@'].contains(&c) || Self::is_whitespace(c);

        loop {
            match self.peek() {
                None => break,
                Some(c) if is_invalid_char(c) => break,
                _ => {
                    self.consume();
                }
            }
        }

        self.token(TokenType::Text)
    }

    fn is_whitespace(c: char) -> bool {
        c == ' ' || c == '\t' || c == '\n' || c == '\r'
    }

    fn whitespace(&mut self) -> Token {
        loop {
            match self.peek() {
                Some('\n') if self.peek_next() == Some('\n') => break,
                Some(c) if Self::is_whitespace(c) => {
                    self.consume();
                }
                _ => break,
            }
        }

        self.token(TokenType::Whitespace)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consume().map(|curr| match curr {
            '[' => self.token(TokenType::LeftBracket),
            ']' => self.token(TokenType::RightBracket),
            '(' => self.token(TokenType::LeftParen),
            ')' => self.token(TokenType::RightParen),
            '|' => self.token(TokenType::ArgumentSeparator),
            '#' => self.identifier(TokenType::FunctionIdentifier),
            '@' => self.identifier(TokenType::AttributeIdentifier),
            '\n' if matches!(self.peek(), Some('\n')) => {
                self.consume();
                self.token(TokenType::HardLinebreak)
            }
            ' ' | '\t' | '\n' | '\r' => self.whitespace(),
            _ => self.text(),
        })
    }
}
