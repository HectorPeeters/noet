use noet::{
    attribute::Attrs, context::Context, error::Result, evaluator::Evaluator, parser::Parser,
    registry::FunctionRegistry, value::Value, variadic::Variadic,
};

#[derive(Debug, PartialEq)]
pub enum CustomValue {
    Text(String),
    Empty(),
    Linebreak(),
    Block(Vec<CustomValue>),
}

impl<'input> Value<'input> for CustomValue {
    const LINEBREAK: Option<Self> = Some(CustomValue::Linebreak());

    fn from_text_element(text: &'input str) -> Option<Self> {
        Some(Self::Text(text.to_string()))
    }

    fn from_block_element(elements: Vec<Self>) -> Option<Self> {
        Some(Self::Block(elements))
    }
}

#[derive(Default)]
pub struct CustomContext {
    pub version: u32,
    pub value: String,
    pub flag_lang: Option<String>,
    pub flag_export: bool,
    pub variadic_values: Vec<String>,
}

impl Context<CustomValue> for CustomContext {
    fn register_functions(registry: &mut FunctionRegistry<Self, CustomValue>) {
        registry.register_function(func_test, "test");
        registry.register_function(func_version, "version");
        registry.register_function(func_attr, "attr");
        registry.register_function(func_flag_attr, "flag-attr");
        registry.register_function(func_variadic, "variadic");
    }
}

fn func_test(context: &mut CustomContext, _attrs: Attrs, value: String) -> Result<()> {
    context.value = value;
    Ok(())
}

fn func_version(context: &mut CustomContext, _attrs: Attrs, version: u32) -> Result<()> {
    context.version = version;
    Ok(())
}

fn func_attr(context: &mut CustomContext, attrs: Attrs, _value: String) -> Result<()> {
    context.flag_lang = attrs.get_value("lang")?;
    Ok(())
}

fn func_flag_attr(context: &mut CustomContext, attrs: Attrs, _value: String) -> Result<()> {
    context.flag_export = attrs.has_flag("export");
    Ok(())
}

fn func_variadic(context: &mut CustomContext, _attrs: Attrs, args: Variadic<String>) -> Result<()> {
    context.variadic_values = args.into();
    Ok(())
}

#[test]
fn evaluate_text() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("This is some simple text.");

    let evaluator = Evaluator::new();
    let evaluated_document = evaluator.evaluate_document(&mut context, parser)?;

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

    let evaluator = Evaluator::new();
    evaluator.evaluate_document(&mut context, parser)?;

    assert_eq!(context.value, "test");

    Ok(())
}

#[test]
fn evaluate_single_argument_function_numeric() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#version 1234]");

    let evaluator = Evaluator::new();
    evaluator.evaluate_document(&mut context, parser)?;

    assert_eq!(context.version, 1234);

    Ok(())
}

#[test]
fn evaluate_single_attribute_function() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#attr @lang(rust) some code]");

    let evaluator = Evaluator::new();
    evaluator.evaluate_document(&mut context, parser)?;

    assert_eq!(context.flag_lang, Some("rust".to_string()));

    Ok(())
}

#[test]
fn evaluate_single_flag_attribute_function() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#flag-attr @export some text]");

    let evaluator = Evaluator::new();
    evaluator.evaluate_document(&mut context, parser)?;

    assert!(context.flag_export);

    Ok(())
}

#[test]
fn evaluate_single_flag_attribute_function_not_present() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#flag-attr Some text]");

    let evaluator = Evaluator::new();
    evaluator.evaluate_document(&mut context, parser)?;

    assert!(!context.flag_export);

    Ok(())
}

#[test]
fn evaluate_variadic() -> Result<()> {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#variadic first | second | third]");

    let evaluator = Evaluator::new();
    evaluator.evaluate_document(&mut context, parser)?;

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
