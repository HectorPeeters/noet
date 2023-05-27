use std::iter::Peekable;

use crate::{
    attribute::Attribute,
    error::{Error, Result},
    lexer::{Lexer, Span, Token, TokenType},
    parse_tree::ParsedElement,
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
    fn consume_expect(&mut self, token_type: TokenType) -> Result<Token> {
        let token = self.consume();

        let Some(token) = token else {
            return Err(Error::Parse("Reached EOF".to_string(), None));
        };

        if token.token_type != token_type {
            return Err(Error::Parse(
                format!(
                    "Expected token {:?} but got {:?}",
                    token_type, token.token_type
                ),
                Some(token.span),
            ));
        }

        Ok(token)
    }

    #[inline]
    fn skip_whitespace(&mut self) {
        while self.peek_type() == Some(TokenType::Whitespace) {
            // NOTE: unwrapping here is allowed as we first check the next token type before consuming
            self.consume_expect(TokenType::Whitespace).unwrap();
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
    fn text(&mut self, start_with_paren: bool) -> ParsedElement<'input> {
        let mut paren_depth = start_with_paren as u32;

        loop {
            match self.peek_type() {
                Some(TokenType::Text | TokenType::Whitespace) => {
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
    fn attribute(&mut self) -> Result<Attribute<'input>> {
        let key = self.consume_expect(TokenType::AttributeIdentifier)?;
        let key_str = &self.input[key.span].trim_start_matches(|c| c == '@');

        match self.peek_type() {
            Some(TokenType::Whitespace | TokenType::RightBracket) => {
                Ok(Attribute::new_flag(key_str))
            }
            Some(TokenType::LeftParen) => {
                self.consume_expect(TokenType::LeftParen)?;

                self.start_span();

                let value_element = self.text(false);

                self.consume_expect(TokenType::RightParen)?;

                if let ParsedElement::Text(value) = value_element {
                    Ok(Attribute::new_value(key_str, value))
                } else {
                    Err(Error::Parse(
                        "Value of attribute should be a string".to_string(),
                        Some(self.get_span()),
                    ))
                }
            }
            x => Err(Error::Parse(
                format!("Unexpected token while parsing attribute {x:?}"),
                Some(self.get_span()),
            )),
        }
    }

    fn trim_argument(elements: &mut Vec<ParsedElement>) -> bool {
        if let Some(ParsedElement::Text(t)) = elements.first_mut() {
            *t = t.trim_start();
            if t.is_empty() {
                elements.remove(0);
            }
        }

        if let Some(ParsedElement::Text(t)) = elements.last_mut() {
            *t = t.trim_end();
            if t.is_empty() {
                elements.pop();
            }
        }

        !elements.is_empty()
    }

    #[inline]
    fn function(&mut self) -> Result<ParsedElement<'input>> {
        let identifier = self.consume_expect(TokenType::FunctionIdentifier)?;

        self.skip_whitespace();

        let mut attributes = vec![];
        let mut arguments = vec![];

        while self.peek_type() == Some(TokenType::AttributeIdentifier) {
            attributes.push(self.attribute()?);
            self.skip_whitespace();
        }

        if let Some(TokenType::ArgumentSeparator) = self.peek_type() {
            self.consume_expect(TokenType::ArgumentSeparator)?;
        }

        while self.peek_type() != Some(TokenType::RightBracket) {
            let mut argument = self.block()?;
            Self::trim_argument(&mut argument);

            if argument.len() == 1 {
                arguments.push(argument.remove(0));
            } else {
                arguments.push(ParsedElement::Block(argument));
            }

            if let Some(TokenType::ArgumentSeparator) = self.peek_type() {
                self.consume_expect(TokenType::ArgumentSeparator)?;
            } else if self.peek_type() != Some(TokenType::RightBracket) {
                return Err(Error::Parse(
                    "Expected RightBracket at the end of function arguments".to_string(),
                    None,
                ));
            }
        }

        self.consume_expect(TokenType::RightBracket)?;

        let _span = self.get_span();

        Ok(ParsedElement::Function(
            self.input[identifier.span].trim_start_matches(|c| c == '#'),
            attributes,
            arguments,
        ))
    }

    #[inline]
    fn block(&mut self) -> Result<Vec<ParsedElement<'input>>> {
        let mut elements = vec![];

        loop {
            let Some(token_type) = self.peek_type() else {
                 break;
            };

            match token_type {
                TokenType::AttributeIdentifier
                | TokenType::ArgumentSeparator
                | TokenType::RightBracket => break,
                _ => match self.element() {
                    Some(e) => elements.push(e?),
                    None => break,
                },
            }
        }

        Ok(elements)
    }

    fn element(&mut self) -> Option<Result<ParsedElement<'input>>> {
        self.start_span();

        let Some(token) = self.consume() else {
            return None;
        };

        match token.token_type {
            TokenType::Text | TokenType::Whitespace => Some(Ok(self.text(false))),
            TokenType::LeftParen => Some(Ok(self.text(true))),
            TokenType::HardLinebreak => Some(Ok(ParsedElement::HardLinebreak())),
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
    type Item = Result<ParsedElement<'input>>;

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
            Some(Ok(ParsedElement::Text("This is some simple text.")))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn matching_paren_text() {
        let mut parser = Parser::new("This is some (simple) text.");

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Text("This is some (simple) text.")))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn matching_paren_start_text() {
        let mut parser = Parser::new("(simple)");

        assert_eq!(parser.next(), Some(Ok(ParsedElement::Text("(simple)"))));
        assert!(parser.next().is_none());
    }

    #[test]
    fn paragraph() {
        let mut parser = Parser::new("This is some simple text.\n\nAnd this is a new paragraph.");

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Text("This is some simple text.")))
        );
        assert_eq!(parser.next(), Some(Ok(ParsedElement::HardLinebreak())));
        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Text("And this is a new paragraph.")))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function() {
        let mut parser = Parser::new("[#test first | second]");

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![ParsedElement::Text("first"), ParsedElement::Text("second")]
            )))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function_arguments() {
        let mut parser = Parser::new("[#title Test Document]\n[#authors John Doe | Jane Doe]");

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "title",
                vec![],
                vec![ParsedElement::Text("Test Document"),]
            )))
        );
        assert_eq!(parser.next(), Some(Ok(ParsedElement::Text("\n"))));
        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "authors",
                vec![],
                vec![
                    ParsedElement::Text("John Doe"),
                    ParsedElement::Text("Jane Doe")
                ]
            )))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn function_attribute() {
        let mut parser = Parser::new("[#test @abc @def(ghi) first | second]");

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "test",
                vec![
                    Attribute::new_flag("abc"),
                    Attribute::new_value("def", "ghi")
                ],
                vec![ParsedElement::Text("first"), ParsedElement::Text("second")]
            )))
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
            Some(Ok(ParsedElement::Function(
                "quote",
                vec![],
                vec![ParsedElement::Block(vec![
                    ParsedElement::Text("Some quote..."),
                    ParsedElement::HardLinebreak(),
                    ParsedElement::Text("Spread over multiple paragraphs.\nBecause edgecases!"),
                ])]
            )))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn nested_functions() {
        let mut parser = Parser::new("[#list [#mi \\lambda x.M] |\n[#mi (M\\;N)]]");

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "list",
                vec![],
                vec![
                    ParsedElement::Function(
                        "mi",
                        vec![],
                        vec![ParsedElement::Text("\\lambda x.M")]
                    ),
                    ParsedElement::Function("mi", vec![], vec![ParsedElement::Text("(M\\;N)")])
                ]
            )))
        );
        assert!(parser.next().is_none());
    }

    #[test]
    fn space_after_paragraph() {
        let mut parser = Parser::new("Test \n\nTest");

        assert_eq!(parser.next(), Some(Ok(ParsedElement::Text("Test "))));
        assert_eq!(parser.next(), Some(Ok(ParsedElement::HardLinebreak())));
        assert_eq!(parser.next(), Some(Ok(ParsedElement::Text("Test"))));
        assert!(parser.next().is_none());
    }

    #[test]
    fn double_whitespace_between_functions() {
        let mut parser = Parser::new(
            r#"[#title This is some document]

[#table @cols(2) @header
| Name | Score
| Apple | 4
| Banana | 8
| Pear | 9
]"#,
        );

        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "title",
                vec![],
                vec![ParsedElement::Text("This is some document")]
            )))
        );
        assert_eq!(parser.next(), Some(Ok(ParsedElement::HardLinebreak())));
        assert_eq!(
            parser.next(),
            Some(Ok(ParsedElement::Function(
                "table",
                vec![
                    Attribute::new_value("cols", "2"),
                    Attribute::new_flag("header")
                ],
                vec![
                    ParsedElement::Text("Name"),
                    ParsedElement::Text("Score"),
                    ParsedElement::Text("Apple"),
                    ParsedElement::Text("4"),
                    ParsedElement::Text("Banana"),
                    ParsedElement::Text("8"),
                    ParsedElement::Text("Pear"),
                    ParsedElement::Text("9"),
                ]
            )))
        );
        assert!(parser.next().is_none());
    }
}
