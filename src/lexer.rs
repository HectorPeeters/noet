use std::{ops::Range, str::Chars};

use itertools::{Itertools, MultiPeek};

pub type Span = Range<usize>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenType {
    Text,
    LeftBracket,
    RightBracket,
    FunctionIdentifier,
    ArgumentSeparator,
    ParagraphBreak,
    Error,
}

#[derive(Debug, PartialEq)]
pub struct Token<'input> {
    pub token_type: TokenType,
    pub value: &'input str,
    pub span: Span,
}

impl<'input> Token<'input> {
    pub fn new(token_type: TokenType, value: &'input str, span: Span) -> Self {
        Self {
            token_type,
            value,
            span,
        }
    }
}

pub struct Lexer<'input> {
    input: &'input str,
    chars: MultiPeek<Chars<'input>>,
    start: usize,
    current: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            chars: input.chars().multipeek(),
            start: 0,
            current: 0,
        }
    }

    fn token(&mut self, token_type: TokenType) -> Token<'input> {
        let span = self.start..self.current;
        self.start = self.current;
        Token::new(token_type, &self.input[span.clone()], span)
    }

    fn consume(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn function_identifier(&mut self) -> Token<'input> {
        let is_valid_char = |c: char| c.is_alphabetic() || c.is_numeric() || c == '-';

        loop {
            match self.peek() {
                Some(c) if is_valid_char(c) => {
                    self.consume();
                }
                _ => break,
            }
        }

        self.token(TokenType::FunctionIdentifier)
    }

    fn text(&mut self) -> Token<'input> {
        let is_invalid_char = |c: char| c == '[' || c == ']' || c == '|' || c == '#';

        loop {
            self.chars.reset_peek();

            match self.peek() {
                None => break,
                Some(c) if is_invalid_char(c) => break,
                Some('\n') if self.peek() == Some('\n') => break,
                _ => {
                    self.consume();
                }
            }
        }

        self.token(TokenType::Text)
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Token<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(curr) = self.consume() else {
            return None;
        };

        match curr {
            '[' => Some(self.token(TokenType::LeftBracket)),
            ']' => Some(self.token(TokenType::RightBracket)),
            '|' => Some(self.token(TokenType::ArgumentSeparator)),
            '#' => Some(self.function_identifier()),
            '\n' if self.peek() == Some('\n') => {
                self.consume();
                Some(self.token(TokenType::ParagraphBreak))
            }
            _ => Some(self.text()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_text() {
        let input = "This is some simple text.";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenType::Text,
                "This is some simple text.",
                0..25
            ))
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn paragraph_break() {
        let input = "This is some simple text\nthat contains a single newline.\n\nThis is some text on a new paragraph.";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenType::Text,
                "This is some simple text\nthat contains a single newline.",
                0..56
            ))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::ParagraphBreak, "\n\n", 56..58))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenType::Text,
                "This is some text on a new paragraph.",
                58..95
            ))
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn special_characters() {
        let input = "[|]#";
        let mut lexer = Lexer::new(input);
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::LeftBracket, "[", 0..1))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::ArgumentSeparator, "|", 1..2))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::RightBracket, "]", 2..3))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::FunctionIdentifier, "#", 3..4))
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn function_identifier() {
        let input = "#this-is-some-identifier";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next(),
            Some(Token::new(
                TokenType::FunctionIdentifier,
                "#this-is-some-identifier",
                0..24
            ))
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn function() {
        let input = "[#list First | Second]";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::LeftBracket, "[", 0..1))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::FunctionIdentifier, "#list", 1..6))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::Text, " First ", 6..13))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::ArgumentSeparator, "|", 13..14))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::Text, " Second", 14..21))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::RightBracket, "]", 21..22))
        );
        assert!(lexer.next().is_none());
    }

    #[test]
    fn function_args_newline() {
        let input = "[#list\n| First\n| Second\n]";
        let mut lexer = Lexer::new(input);

        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::LeftBracket, "[", 0..1))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::FunctionIdentifier, "#list", 1..6))
        );
        assert_eq!(lexer.next(), Some(Token::new(TokenType::Text, "\n", 6..7)));
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::ArgumentSeparator, "|", 7..8))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::Text, " First\n", 8..15))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::ArgumentSeparator, "|", 15..16))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::Text, " Second\n", 16..24))
        );
        assert_eq!(
            lexer.next(),
            Some(Token::new(TokenType::RightBracket, "]", 24..25))
        );
        assert!(lexer.next().is_none());
    }
}
