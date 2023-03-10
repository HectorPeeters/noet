use noet::{
    attribute::Attrs, context::Context, error::Result, evaluator::Evaluator, parser::Parser,
    registry::FunctionRegistry, value::Value, variadic::Variadic,
};

#[derive(Debug, PartialEq)]
pub enum Element {
    Text(String),
    Bold(Box<Element>),
    List(Vec<Element>),
    Block(Vec<Element>),
    Table(Vec<Element>, u32, bool),
    Linebreak(),
}

impl<'input> Value<'input> for Element {
    const LINEBREAK: Option<Self> = Some(Element::Linebreak());

    fn from_text_element(text: &'input str) -> Option<Self> {
        Some(Self::Text(text.to_string()))
    }

    fn from_block_element(elements: Vec<Self>) -> Option<Self> {
        Some(Self::Block(elements))
    }
}

#[derive(Default)]
pub struct Note {
    pub title: Option<String>,
}

impl Context<Element> for Note {
    fn register_functions(registry: &mut FunctionRegistry<Self, Element>) {
        registry.register_function(func_title, "title");
        registry.register_function(func_bold, "b");
        registry.register_function(func_list, "list");
        registry.register_function(func_table, "table");
    }
}

fn func_title(context: &mut Note, _attrs: Attrs, title: String) {
    context.title = Some(title);
}

fn func_bold(_context: &mut Note, _attrs: Attrs, elem: Element) -> Element {
    Element::Bold(Box::new(elem))
}

fn func_list(_context: &mut Note, _attrs: Attrs, items: Variadic<Element>) -> Element {
    Element::List(items.into())
}

fn func_table(_context: &mut Note, attrs: Attrs, items: Variadic<Element>) -> Result<Element> {
    Ok(Element::Table(
        items.into(),
        attrs.get_value("cols")?.unwrap_or(1),
        attrs.has_flag("header"),
    ))
}

fn parse_document(doc: &str) -> Result<(Note, Vec<Element>)> {
    let parser = Parser::new(doc);

    let mut context = Note::default();

    let evaluator = Evaluator::new();
    let evaluated = evaluator.evaluate_document(&mut context, parser)?;

    Ok((context, evaluated))
}

#[test]
fn full_document() -> Result<()> {
    let source = r#"[#title This is some document]

It contains a [#b first] paragraph and supports lists.

[#list
| first
| second
| [#b third]
]

It also supports tables!

[#table @cols(2) @header
| Name | Score
| Apple | 4
| Banana | 8
| Pear | 9
]"#;

    let (context, elements) = parse_document(source)?;

    assert_eq!(context.title, Some("This is some document".to_string()));
    assert_eq!(
        elements,
        vec![
            Element::Linebreak(),
            Element::Text("It contains a ".to_string()),
            Element::Bold(Box::new(Element::Text("first".to_string()))),
            Element::Text(" paragraph and supports lists.".to_string()),
            Element::Linebreak(),
            Element::List(vec![
                Element::Text("first".to_string()),
                Element::Text("second".to_string()),
                Element::Bold(Box::new(Element::Text("third".to_string()))),
            ]),
            Element::Linebreak(),
            Element::Text("It also supports tables!".to_string()),
            Element::Linebreak(),
            Element::Table(
                vec![
                    Element::Text("Name".to_string()),
                    Element::Text("Score".to_string()),
                    Element::Text("Apple".to_string()),
                    Element::Text("4".to_string()),
                    Element::Text("Banana".to_string()),
                    Element::Text("8".to_string()),
                    Element::Text("Pear".to_string()),
                    Element::Text("9".to_string()),
                ],
                2,
                true
            )
        ]
    );

    Ok(())
}
