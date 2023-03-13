use noet::{
    argument::Argument, attribute::Attrs, context::Context, evaluator::Evaluator,
    parse_tree::ParsedElement, parser::Parser, registry::FunctionRegistry, value::Value,
};

#[derive(Debug, Clone, PartialEq)]
pub enum CustomValue {
    Text(String),
    Empty(),
    Linebreak(),
}

impl<'input> From<ParsedElement<'input>> for CustomValue {
    fn from(value: ParsedElement<'input>) -> Self {
        match value {
            ParsedElement::Text(t) => CustomValue::Text(t.to_string()),
            ParsedElement::HardLinebreak() => CustomValue::Linebreak(),
            ParsedElement::Function(_, _, _) => unreachable!(),
        }
    }
}

impl<'input> Value<'input> for CustomValue {}

impl<'a> Argument<'a, CustomValue> for String {
    fn from_block(value: &'a [CustomValue]) -> Option<Self> {
        match &value[0] {
            CustomValue::Text(t) => Some(t.to_string()),
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
        //        registry.register_function(func_variadic, "variadic");
    }
}

fn func_test(context: &mut CustomContext, _attrs: &Attrs, value: String) -> Option<CustomValue> {
    context.value = value;
    None
}

fn func_attr(context: &mut CustomContext, attrs: &Attrs, _value: String) -> Option<CustomValue> {
    context.flag_lang = attrs.get_value("lang").map(|x| x.to_string());
    None
}

fn func_flag_attr(
    context: &mut CustomContext,
    attrs: &Attrs,
    _value: String,
) -> Option<CustomValue> {
    context.flag_export = attrs.has_flag("export");
    None
}

// fn func_variadic(context: &mut CustomContext, args: Variadic<String>) -> Option<CustomValue> {
//     context.variadic_values = args.into_inner();
//     None
// }

#[test]
fn evaluate_text() {
    let mut context = CustomContext::default();

    let parser = Parser::new("This is some simple text.");

    let mut evaluator = Evaluator::new(&mut context);
    let evaluated_document = evaluator.evaluate_document(parser);

    assert_eq!(
        evaluated_document,
        vec![CustomValue::Text("This is some simple text.".to_string()),]
    );
}

#[test]
fn evaluate_single_argument_function() {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#test test]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser);

    assert_eq!(context.value, "test");
}

#[test]
fn evaluate_single_attribute_function() {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#attr @lang(rust) some code]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser);

    assert_eq!(context.flag_lang, Some("rust".to_string()));
}

#[test]
fn evaluate_single_flag_attribute_function() {
    let mut context = CustomContext::default();

    let parser = Parser::new("[#flag-attr @export some text]");

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(parser);

    assert!(context.flag_export);
}

// #[test]
// fn evaluate_single_flag_attribute_function_not_present() {
//     let mut context = CustomContext::default();
//
//     let document = parser::note("[#flag-attr]").unwrap();
//
//     let mut evaluator = Evaluator::new(&mut context);
//     evaluator.evaluate_document(document);
//
//     assert!(!context.flag_export);
// }
//
// #[test]
// fn evaluate_variadic() {
//     let mut context = CustomContext::default();
//
//     let document = parser::note("[#variadic first | second | third]").unwrap();
//
//     let mut evaluator = Evaluator::new(&mut context);
//     evaluator.evaluate_document(document);
//
//     assert_eq!(
//         context.variadic_values,
//         vec![
//             "first".to_string(),
//             "second".to_string(),
//             "third".to_string()
//         ]
//     );
// }
