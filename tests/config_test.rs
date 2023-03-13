use noet::{
    argument::Argument, attribute::Attrs, context::Context, error::Result, evaluator::Evaluator,
    parse_tree::ParsedElement, parser::Parser, registry::FunctionRegistry, value::Value,
    variadic::Variadic,
};

#[derive(Debug, Clone, PartialEq)]
pub enum CustomValue {
    Text(String),
    Empty(),
    Linebreak(),
    Block(Vec<CustomValue>),
}

impl<'input> From<ParsedElement<'input>> for CustomValue {
    fn from(value: ParsedElement<'input>) -> Self {
        match value {
            ParsedElement::Text(t) => CustomValue::Text(t.to_string()),
            ParsedElement::HardLinebreak() => CustomValue::Linebreak(),
            ParsedElement::Block(elems) => {
                CustomValue::Block(elems.into_iter().map(Into::into).collect())
            }
            ParsedElement::Function(_, _, _) => unreachable!(),
        }
    }
}

impl<'input> Value<'input> for CustomValue {}

impl<'a> Argument<'a, CustomValue> for String {
    fn from_value(value: &CustomValue) -> Self {
        match value {
            CustomValue::Text(t) => t.to_string(),
            CustomValue::Block(elems) => {
                if elems.len() != 1 {
                    panic!();
                }

                Argument::from_value(&elems[0])
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Default)]
pub struct CustomContext {
    pub value: String,
    pub flag_lang: Option<String>,
    pub flag_export: bool,
    pub variadic_values: Vec<String>,
}

impl Context<CustomValue> for CustomContext {
    fn register_functions(registry: &mut FunctionRegistry<Self, CustomValue>) {
        registry.register_function(func_test, "test");
        registry.register_function(func_attr, "attr");
        registry.register_function(func_flag_attr, "flag-attr");
        registry.register_function(func_variadic, "variadic");
    }
}

fn func_test(
    context: &mut CustomContext,
    _attrs: Attrs,
    value: String,
) -> Result<Option<CustomValue>> {
    context.value = value;
    Ok(None)
}

fn func_attr(
    context: &mut CustomContext,
    attrs: Attrs,
    _value: String,
) -> Result<Option<CustomValue>> {
    context.flag_lang = attrs.get_value("lang")?;
    Ok(None)
}

fn func_flag_attr(
    context: &mut CustomContext,
    attrs: Attrs,
    _value: String,
) -> Result<Option<CustomValue>> {
    context.flag_export = attrs.has_flag("export");
    Ok(None)
}

fn func_variadic(
    context: &mut CustomContext,
    _attrs: Attrs,
    args: Variadic<String>,
) -> Result<Option<CustomValue>> {
    context.variadic_values = args.into();
    Ok(None)
}

#[test]
fn evaluate_text() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("This is some simple text.");

    let mut evaluator = Evaluator::new(&mut context);
    let evaluated_document = evaluator.evaluate_document(parser)?;

    assert_eq!(
        evaluated_document,
        vec![CustomValue::Text("This is some simple text.".to_string()),]
    );

    Ok(())
}

#[test]
fn evaluate_single_argument_function() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#test test]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser)?;

    assert_eq!(context.value, "test");

    Ok(())
}

#[test]
fn evaluate_single_attribute_function() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#attr @lang(rust) some code]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser)?;

    assert_eq!(context.flag_lang, Some("rust".to_string()));

    Ok(())
}

#[test]
fn evaluate_single_flag_attribute_function() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#flag-attr @export some text]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser)?;

    assert!(context.flag_export);

    Ok(())
}

#[test]
fn evaluate_single_flag_attribute_function_not_present() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#flag-attr Some text]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser)?;

    assert!(!context.flag_export);

    Ok(())
}

#[test]
fn evaluate_variadic() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#variadic first | second | third]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser)?;

    assert_eq!(
        context.variadic_values,
        vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string()
        ]
    );

    Ok(())
}
