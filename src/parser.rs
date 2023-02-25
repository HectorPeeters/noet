use std::iter::Peekable;

use crate::{
    lexer::{Lexer, Span, Token, TokenType},
    parse_tree::{Block, ParsedElement},
};

pub struct Parser<'input> {
    input: &'input str,
    tokens: Peekable<Lexer<'input>>,
    start: usize,
    current: usize,
}

impl<'input> Parser<'input> {
    pub fn new(input: &'input str) -> Self {
        Self {
            input,
            tokens: Lexer::new(input).peekable(),
            start: 0,
            current: 0,
        }
    }

    fn consume(&mut self) -> Option<Token<'input>> {
        let result = self.tokens.next();
        if let Some(res) = &result {
            self.current = res.span.end;
        }
        result
    }

    fn consume_expect(&mut self, token_type: TokenType) -> Token<'input> {
        let token = self.consume();
        if let Some(token) = token && token.token_type == token_type {
            return token;
        }

        panic!("Expected token of type {token_type:?}");
    }

    fn peek_type(&mut self) -> Option<TokenType> {
        self.tokens.peek().map(|t| t.token_type)
    }

    fn start_span(&mut self) {
        self.start = self.current;
    }

    fn get_span(&mut self) -> Span {
        let result = self.start..self.current;
        result
    }

    fn text(&mut self) -> ParsedElement<'input> {
        let span = self.get_span();
        ParsedElement::Text(&self.input[span])
    }

    fn paragraph_break(&mut self) -> ParsedElement<'input> {
        ParsedElement::ParagraphBreak()
    }

    fn trim_argument(block: &mut Block) -> bool {
        if let Some(ParsedElement::Text(t)) = block.elements.first_mut() {
            *t = t.trim_start();
            if t.is_empty() {
                return false;
            }
        }

        if let Some(ParsedElement::Text(t)) = block.elements.last_mut() {
            *t = t.trim_end();
            if t.is_empty() {
                return false;
            }
        }

        true
    }

    fn function(&mut self) -> ParsedElement<'input> {
        // TODO: check if we have a function identifer, otherwise just parse matching square
        // brackets.
        let identifier = self.consume_expect(TokenType::FunctionIdentifier);

        let mut arguments = vec![];

        loop {
            if self.peek_type() == Some(TokenType::RightBracket) {
                break;
            }

            if self.peek_type().is_none() {
                panic!("Unclosed function brackets");
            }

            if self.peek_type() == Some(TokenType::ArgumentSeparator) {
                self.consume();
            }

            if let Some(mut block) = self.block() {
                if Self::trim_argument(&mut block) {
                    arguments.push(block);
                }
            } else {
                panic!("Something weird happened");
            }
        }

        self.consume_expect(TokenType::RightBracket);

        let _span = self.get_span();

        ParsedElement::Function(&self.input[identifier.span], arguments)
    }

    fn element(&mut self) -> Option<ParsedElement<'input>> {
        self.start_span();

        let Some(token) = self.consume() else {
            return None;
        };

        match token.token_type {
            TokenType::Text => Some(self.text()),
            TokenType::LeftBracket => Some(self.function()),
            TokenType::ParagraphBreak => Some(self.paragraph_break()),
            TokenType::RightBracket
            | TokenType::FunctionIdentifier
            | TokenType::ArgumentSeparator => panic!("Invalid token found {:?}", token.token_type),
            TokenType::Error => panic!("Do some better error handling"),
        }
    }

    fn block(&mut self) -> Option<Block<'input>> {
        let mut elements = vec![];

        if self.peek_type().is_none() {
            return None;
        }

        loop {
            let Some(token_type) = self.peek_type() else {
                break;
            };

            match token_type {
                TokenType::ArgumentSeparator | TokenType::RightBracket => break,
                _ => match self.element() {
                    Some(e) => elements.push(e),
                    None => break,
                },
            }
        }

        let _span = self.get_span();

        Some(Block::new(elements))
    }
}

impl<'input> Iterator for Parser<'input> {
    type Item = ParsedElement<'input>;

    fn next(&mut self) -> Option<Self::Item> {
        self.element()
    }
}
