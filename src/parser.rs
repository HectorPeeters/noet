use crate::parse_tree::{ParsedAttribute, ParsedDocument, ParsedElement};

peg::parser!(pub grammar parser() for str {
    rule text_end()
        = paragraph_end()
        / argument_separator()
        / function_end()
        / function_start()

    rule matching_square_brackets() -> &'input str
        = $("[" text() "]")

    rule text_component() -> &'input str
        = $(!text_end() (matching_square_brackets() / [_]))

    rule text() -> &'input str
        = $(text_component()*)

    rule text_plus() -> &'input str
        = $(text_component()+)

    rule _()
        = [' ' | '\t' | '\n' | '\r']*

    rule identifier() -> &'input str
        = $(['a'..='z' | 'A'..='Z' | '0'..='9' | '-']+)

    rule function_start() -> &'input str
        = "[#" i:identifier()
        { i }

    rule function_end()
         = "]"

    rule function_arg() -> ParsedElement<'input>
        = function()
        / t:text_plus() { ParsedElement::Text(t.trim()) }

    rule argument_separator()
        = _ "|" _

    rule attribute_argument() -> ParsedElement<'input>
        = t:$((!")" [_])*)
        { ParsedElement::Text(t) }

    rule attribute() -> ParsedAttribute<'input>
        = "@" i:identifier() "(" v:attribute_argument() ")"
        { ParsedAttribute::Value(i, v) }

    rule flag_attribute() -> ParsedAttribute<'input>
        = "@" i:identifier()
        { ParsedAttribute::Flag(i) }

    rule attributes() -> Vec<ParsedAttribute<'input>>
        = (attribute() / flag_attribute()) ** _

    pub rule function() -> ParsedElement<'input>
        = s:function_start() _
        attrs:attributes()  _
        argument_separator()? _ args:(function_arg() ** argument_separator())
        _ function_end()
        {
            ParsedElement::Function(s, attrs, args)
        }

    rule paragraph_component() -> ParsedElement<'input>
        = function()
        / t:text_plus() { ParsedElement::Text(t) }

    rule paragraph_break()
        = ("\n\n" / "\r\n\r\n")

    rule paragraph_end() -> Option<ParsedElement<'input>>
        = paragraph_break() { Some(ParsedElement::ParagraphBreak {}) }
        / ![_] { None }

    rule paragraph() -> Vec<ParsedElement<'input>>
        = es:(paragraph_component()+) e:paragraph_end()
        {
            let mut elements = es;
            if let Some(end) = e {
                elements.push(end);
            }
            elements
        }

    pub rule paragraphs() -> Vec<ParsedElement<'input>>
        = p:(paragraph()*)
        { p.into_iter().flatten().collect() }

    pub rule note() -> ParsedDocument<'input>
        = el:paragraphs()
        { ParsedDocument::new(el) }
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let result = parser::paragraphs("");
        assert_eq!(result, Ok(vec![]));
    }

    #[test]
    fn parse_spaces() {
        let result = parser::paragraphs("   ");
        assert_eq!(result, Ok(vec![ParsedElement::Text("   ")]));
    }

    #[test]
    fn parse_single_line_text() {
        let result = parser::paragraphs("This is some text.");
        assert_eq!(result, Ok(vec![ParsedElement::Text("This is some text.",)]));
    }

    #[test]
    fn parse_multi_line_text() {
        let result = parser::paragraphs("This is some text\nwhich continues on another line.");
        assert_eq!(
            result,
            Ok(vec![ParsedElement::Text(
                "This is some text\nwhich continues on another line.",
            )])
        );
    }

    #[test]
    fn parse_single_paragraph() {
        let result = parser::paragraphs("This is some text.\n\n");
        assert_eq!(
            result,
            Ok(vec![
                ParsedElement::Text("This is some text.",),
                ParsedElement::ParagraphBreak(),
            ])
        );
    }

    #[test]
    fn parse_multiple_paragraphs() {
        let result =
            parser::paragraphs("This is some text.\n\nThis is some text on another paragraph.");
        assert_eq!(
            result,
            Ok(vec![
                ParsedElement::Text("This is some text.",),
                ParsedElement::ParagraphBreak(),
                ParsedElement::Text("This is some text on another paragraph.",),
            ])
        );
    }

    #[test]
    fn parse_function_no_args() {
        let result = parser::function("[#test]");
        assert_eq!(result, Ok(ParsedElement::Function("test", vec![], vec![])));
    }

    #[test]
    fn parse_function_one_arg() {
        let result = parser::function("[#test test this is an argument]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![ParsedElement::Text("test this is an argument")]
            ))
        );
    }

    #[test]
    pub fn parse_function_square_bracket_arg_empty() {
        let result = parser::function("[#test This is an [] argument]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![ParsedElement::Text("This is an [] argument")],
            ))
        );
    }

    #[test]
    pub fn parse_function_square_bracket_arg() {
        let result =
            parser::function("[#test This is an [ this also contains some text] argument]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![ParsedElement::Text(
                    "This is an [ this also contains some text] argument"
                )],
            ))
        );
    }

    #[test]
    fn parse_function_flag_attr() {
        let result = parser::function("[#test @some-attribute]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![ParsedAttribute::Flag("some-attribute")],
                vec![]
            ))
        );
    }

    #[test]
    fn parse_function_attr() {
        let result = parser::function("[#test @lang(rust)]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![ParsedAttribute::Value("lang", ParsedElement::Text("rust"))],
                vec![]
            ))
        );
    }

    #[test]
    fn parse_multiple_function_no_space() {
        let result = parser::paragraphs(
            "[#test test this is an argument][#test2 test this is another argument]",
        );
        assert_eq!(
            result,
            Ok(vec![
                ParsedElement::Function(
                    "test",
                    vec![],
                    vec![ParsedElement::Text("test this is an argument")]
                ),
                ParsedElement::Function(
                    "test2",
                    vec![],
                    vec![ParsedElement::Text("test this is another argument")]
                )
            ])
        );
    }

    #[test]
    fn parse_function_in_text() {
        let result = parser::paragraphs("This is some text with a [#b bold] word.");
        assert_eq!(
            result,
            Ok(vec![
                ParsedElement::Text("This is some text with a "),
                ParsedElement::Function("b", vec![], vec![ParsedElement::Text("bold")]),
                ParsedElement::Text(" word."),
            ])
        );
    }

    #[test]
    fn parse_multiple_function() {
        let result = parser::paragraphs(
            "[#test test this is an argument] [#test2 test this is another argument]",
        );
        assert_eq!(
            result,
            Ok(vec![
                ParsedElement::Function(
                    "test",
                    vec![],
                    vec![ParsedElement::Text("test this is an argument")]
                ),
                ParsedElement::Text(" "),
                ParsedElement::Function(
                    "test2",
                    vec![],
                    vec![ParsedElement::Text("test this is another argument")]
                )
            ])
        );
    }

    #[test]
    fn parse_function_two_arg() {
        let result = parser::function("[#test test this is an argument | two arguments]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![
                    ParsedElement::Text("test this is an argument"),
                    ParsedElement::Text("two arguments")
                ]
            ))
        );
    }

    #[test]
    fn parse_function_two_arg_new_lines() {
        let result = parser::function("[#test\ntest this is an argument\n|\ntwo arguments]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![
                    ParsedElement::Text("test this is an argument"),
                    ParsedElement::Text("two arguments")
                ]
            ))
        );
    }

    #[test]
    fn parse_function_two_arg_new_lines_leading_separator() {
        let result = parser::function("[#test\n| test this is an argument\n| two arguments\n]");
        assert_eq!(
            result,
            Ok(ParsedElement::Function(
                "test",
                vec![],
                vec![
                    ParsedElement::Text("test this is an argument"),
                    ParsedElement::Text("two arguments")
                ]
            ))
        );
    }
}
