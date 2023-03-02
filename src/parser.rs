use std::iter::Peekable;

use crate::{
    lexer::{Lexer, Span, Token, TokenType},
    parse_tree::{Attribute, Block, ParsedElement},
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
        self.start..self.current
    }

    fn text(&mut self) -> ParsedElement<'input> {
        let mut paren_depth = 0;

        loop {
            match self.peek_type() {
                Some(TokenType::Text) => {
                    self.consume();
                }
                Some(TokenType::LeftParen) => {
                    paren_depth += 1;
                    self.consume();
                }
                Some(TokenType::RightParen) if paren_depth > 0 => {
                    paren_depth -= 1;
                    self.consume();
                }
                // TODO: add support for parsing square brackets without function identifier
                _ => break,
            }
        }

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
                block.elements.remove(0);
            }
        }

        if let Some(ParsedElement::Text(t)) = block.elements.last_mut() {
            *t = t.trim_end();
            if t.is_empty() {
                block.elements.pop();
            }
        }

        !block.elements.is_empty()
    }

    fn attribute(&mut self) -> Attribute<'input> {
        let identifier = self.consume_expect(TokenType::AttributeIdentifier);

        if self.peek_type() == Some(TokenType::LeftParen) {
            self.consume_expect(TokenType::LeftParen);

            self.start_span();
            let value = self.text();

            let result = if let ParsedElement::Text(text) = value {
                Attribute::new_value(identifier.value, text)
            } else {
                panic!("Expected text element as attribute value");
            };

            self.consume_expect(TokenType::RightParen);

            result
        } else {
            Attribute::new_flag(identifier.value)
        }
    }

    fn function(&mut self) -> ParsedElement<'input> {
        // TODO: check if we have a function identifer, otherwise just parse matching brackets
        let identifier = self.consume_expect(TokenType::FunctionIdentifier);

        let mut attributes = vec![];
        let mut arguments = vec![];

        loop {
            if self.peek_type() == Some(TokenType::RightBracket) {
                break;
            }

            if self.peek_type().is_none() {
                panic!("Unclosed function brackets");
            }

            if self.peek_type() == Some(TokenType::AttributeIdentifier) {
                attributes.push(self.attribute());
                continue;
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

        ParsedElement::Function(
            &self.input[identifier.span].trim_start_matches(|c| c == '#'),
            attributes,
            arguments,
        )
    }

    fn element(&mut self) -> Option<ParsedElement<'input>> {
        self.start_span();

        let Some(token) = self.consume() else {
            return None;
        };

        match token.token_type {
            TokenType::Text | TokenType::LeftParen | TokenType::RightParen => Some(self.text()),
            TokenType::LeftBracket => Some(self.function()),
            TokenType::ParagraphBreak => Some(self.paragraph_break()),
            TokenType::RightBracket
            | TokenType::AttributeIdentifier
            | TokenType::FunctionIdentifier
            | TokenType::ArgumentSeparator => {
                panic!("Invalid token found {:?}", token.token_type)
            }
            TokenType::Error => panic!("Do some better error handling"),
        }
    }

    fn block(&mut self) -> Option<Block<'input>> {
        let mut elements = vec![];

        self.peek_type()?;

        loop {
            let Some(token_type) = self.peek_type() else {
                break;
            };

            match token_type {
                TokenType::AttributeIdentifier
                | TokenType::ArgumentSeparator
                | TokenType::RightBracket => break,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_text() {
        let mut parser = Parser::new("This is some simple text.");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("This is some simple text."))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn paragraph() {
        let mut parser = Parser::new("This is some simple text.\n\nAnd this is a new paragraph.");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("This is some simple text."))
        );
        assert_eq!(parser.next(), Some(ParsedElement::ParagraphBreak()));
        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("And this is a new paragraph."))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn paren_text() {
        let mut parser = Parser::new("This is some (simple) text.");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("This is some (simple) text."))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function() {
        let mut parser = Parser::new("[#test first | second]");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "test",
                vec![],
                vec![
                    Block::new(vec![ParsedElement::Text("first")]),
                    Block::new(vec![ParsedElement::Text("second")])
                ]
            ))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function_arguments() {
        let mut parser = Parser::new("[#title Test Document]\n[#authors John Doe | Jane Doe]");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "title",
                vec![],
                vec![Block::new(vec![ParsedElement::Text("Test Document")]),]
            ))
        );
        assert_eq!(parser.next(), Some(ParsedElement::Text("\n")));
        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "authors",
                vec![],
                vec![
                    Block::new(vec![ParsedElement::Text("John Doe")]),
                    Block::new(vec![ParsedElement::Text("Jane Doe")]),
                ]
            ))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function_attribute() {
        let mut parser = Parser::new("[#test @abc @def(ghi) first | second]");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "test",
                vec![
                    Attribute::new_flag("@abc"),
                    Attribute::new_value("@def", "ghi")
                ],
                vec![
                    Block::new(vec![ParsedElement::Text("first")]),
                    Block::new(vec![ParsedElement::Text("second")])
                ]
            ))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function_multiline_argument() {
        let mut parser = Parser::new(
            "[#quote\nSome quote...\n\nSpread over multiple paragraphs.\nBecause edgecases!\n]",
        );

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "quote",
                vec![],
                vec![Block {
                    elements: vec![
                        ParsedElement::Text("Some quote..."),
                        ParsedElement::ParagraphBreak(),
                        ParsedElement::Text("Spread over multiple paragraphs.\nBecause edgecases!"),
                    ]
                }]
            ))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn nested_functions() {
        let mut parser = Parser::new("[#list [#mi \\lambda x.M]]");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "list",
                vec![],
                vec![Block {
                    elements: vec![ParsedElement::Function(
                        "mi",
                        vec![],
                        vec![Block {
                            elements: vec![ParsedElement::Text("\\lambda x.M")]
                        }]
                    )]
                }]
            ))
        );
        assert!(parser.next().is_none());
    }
}
