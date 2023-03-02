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

    #[inline]
    fn consume(&mut self) -> Option<Token> {
        let result = self.tokens.next();
        if let Some(res) = &result {
            self.current = res.span.end;
        }

        result
    }

    #[inline]
    fn consume_expect(&mut self, token_type: TokenType) -> Token {
        let token = self.consume();
        if let Some(token) = token && token.token_type == token_type {
            return token;
        }

        panic!("Expected token of type {token_type:?}");
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.peek_type() == Some(TokenType::Whitespace) {
            self.consume_expect(TokenType::Whitespace);
        }
    }

    #[inline]
    fn peek_type(&mut self) -> Option<TokenType> {
        self.tokens.peek().map(|t| t.token_type)
    }

    #[inline]
    fn start_span(&mut self) {
        self.start = self.current;
    }

    #[inline]
    fn get_span(&mut self) -> Span {
        self.start..self.current
    }

    #[inline]
    fn text(&mut self, start_with_parent: bool) -> ParsedElement<'input> {
        let mut paren_depth = start_with_parent as u32;

        loop {
            match self.peek_type() {
                Some(TokenType::Text) | Some(TokenType::Whitespace) => {
                    self.consume();
                }
                Some(TokenType::LeftParen) => {
                    self.consume();
                    paren_depth += 1;
                }
                Some(TokenType::RightParen) if paren_depth > 0 => {
                    self.consume();
                    paren_depth -= 1;
                }
                // TODO: add support for parsing square brackets without function identifier
                _ => break,
            }
        }

        let span = self.get_span();
        ParsedElement::Text(&self.input[span])
    }

    #[inline]
    fn attribute(&mut self) -> Attribute<'input> {
        let key = self.consume_expect(TokenType::AttributeIdentifier);

        match self.peek_type() {
            Some(TokenType::Whitespace) => Attribute::new_flag(&self.input[key.span]),
            Some(TokenType::LeftParen) => {
                self.consume_expect(TokenType::LeftParen);

                self.start_span();

                let value_element = self.text(false);

                self.consume_expect(TokenType::RightParen);

                if let ParsedElement::Text(value) = value_element {
                    Attribute::new_value(&self.input[key.span], value)
                } else {
                    panic!("Value of attribute should be a string");
                }
            }
            x @ _ => panic!("Unexpected token while parsing attribute {:?}", x),
        }
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

    #[inline]
    fn function(&mut self) -> ParsedElement<'input> {
        let identifier = self.consume_expect(TokenType::FunctionIdentifier);

        println!("{:?}", &self.input[identifier.span.clone()]);

        self.skip_whitespace();

        let mut attributes = vec![];
        let mut arguments = vec![];

        while self.peek_type() == Some(TokenType::AttributeIdentifier) {
            attributes.push(self.attribute());
            self.skip_whitespace();
        }

        if let Some(TokenType::ArgumentSeparator) = self.peek_type() {
            self.consume_expect(TokenType::ArgumentSeparator);
        }

        while self.peek_type() != Some(TokenType::RightBracket) {
            println!("Parsing attribute");
            let mut argument = self.block();
            Self::trim_argument(&mut argument);
            arguments.push(argument);

            if let Some(TokenType::ArgumentSeparator) = self.peek_type() {
                self.consume_expect(TokenType::ArgumentSeparator);
            } else {
                // TODO: peek_type() should return a RightBracket
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

    #[inline]
    fn block(&mut self) -> Block<'input> {
        let mut elements = vec![];

        loop {
            let Some(token_type) = self.peek_type() else {
                 break;
            };

            println!("B {:?}", token_type);

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

        Block::new(elements)
    }

    fn element(&mut self) -> Option<ParsedElement<'input>> {
        self.start_span();

        let Some(token) = self.consume() else {
            return None;
        };

        match token.token_type {
            TokenType::Text | TokenType::Whitespace => Some(self.text(false)),
            TokenType::LeftParen => Some(self.text(true)),
            TokenType::HardLinebreak => Some(ParsedElement::HardLinebreak()),
            TokenType::LeftBracket => Some(self.function()),
            TokenType::RightBracket => todo!(),
            TokenType::RightParen => todo!(),
            TokenType::AttributeIdentifier => todo!(),
            TokenType::FunctionIdentifier => todo!(),
            TokenType::ArgumentSeparator => todo!(),
            TokenType::Error => todo!(),
        }
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
    fn matching_paren_text() {
        let mut parser = Parser::new("This is some (simple) text.");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("This is some (simple) text."))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn matching_paren_start_text() {
        let mut parser = Parser::new("(simple)");

        assert_eq!(parser.next(), Some(ParsedElement::Text("(simple)")));
        assert!(parser.next().is_none());
    }

    #[test]
    fn paragraph() {
        let mut parser = Parser::new("This is some simple text.\n\nAnd this is a new paragraph.");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("This is some simple text."))
        );
        assert_eq!(parser.next(), Some(ParsedElement::HardLinebreak()));
        assert_eq!(
            parser.next(),
            Some(ParsedElement::Text("And this is a new paragraph."))
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
                        ParsedElement::HardLinebreak(),
                        ParsedElement::Text("Spread over multiple paragraphs.\nBecause edgecases!"),
                    ]
                }]
            ))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn nested_functions() {
        let mut parser = Parser::new("[#list [#mi \\lambda x.M] |\n[#mi (M\\;N)]]");

        assert_eq!(
            parser.next(),
            Some(ParsedElement::Function(
                "list",
                vec![],
                vec![
                    Block {
                        elements: vec![ParsedElement::Function(
                            "mi",
                            vec![],
                            vec![Block {
                                elements: vec![ParsedElement::Text("\\lambda x.M")]
                            }]
                        )]
                    },
                    Block {
                        elements: vec![ParsedElement::Function(
                            "mi",
                            vec![],
                            vec![Block {
                                elements: vec![ParsedElement::Text("(M\\;N)")]
                            }]
                        )]
                    }
                ]
            ))
        );
        assert!(parser.next().is_none());
    }
}
