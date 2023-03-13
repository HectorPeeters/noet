use toen::{
    argument::Argument, attribute::Attribute, context::Context, evaluator::Evaluator,
    parse_tree::ParsedElement, parser::parser, registry::FunctionRegistry, value::Value,
    variadic::Variadic,
};

pub enum CustomValue {
    Text(String),
    Empty(),
}

impl<'input> Value<'input> for CustomValue {
    fn from_element(element: &ParsedElement<'input>) -> Option<Self> {
        match element {
            ParsedElement::Text(t) => Some(CustomValue::Text(t.to_string())),
            _ => None,
        }
    }

    fn empty() -> Self {
        CustomValue::Empty()
    }
}

impl<'a> Argument<'a, CustomValue> for String {
    fn from_value(value: &'a CustomValue) -> Option<Self> {
        match value {
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
        registry.register_function(func_variadic, "variadic");
    }
}

fn func_test(context: &mut CustomContext, value: String) -> Option<CustomValue> {
    context.value = value;
    None
}

fn func_attr(context: &mut CustomContext, lang: Attribute<"lang", String>) -> Option<CustomValue> {
    context.flag_lang = lang.into_inner();
    None
}

fn func_flag_attr(
    context: &mut CustomContext,
    lang: Attribute<"export", ()>,
) -> Option<CustomValue> {
    context.flag_export = lang.into_inner().is_some();
    None
}

fn func_variadic(context: &mut CustomContext, args: Variadic<String>) -> Option<CustomValue> {
    context.variadic_values = args.into_inner();
    None
}

#[test]
fn evaluate_single_argument_function() {
    let mut context = CustomContext::default();

    let document = parser::note("[#test test]").unwrap();

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(document);

    assert_eq!(context.value, "test");
}

#[test]
fn evaluate_single_attribute_function() {
    let mut context = CustomContext::default();

    let document = parser::note("[#attr @lang(rust)]").unwrap();

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(document);

    assert_eq!(context.flag_lang, Some("rust".to_string()));
}

#[test]
fn evaluate_single_flag_attribute_function() {
    let mut context = CustomContext::default();

    let document = parser::note("[#flag-attr @export]").unwrap();

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(document);

    assert!(context.flag_export);
}

#[test]
fn evaluate_single_flag_attribute_function_not_present() {
    let mut context = CustomContext::default();

    let document = parser::note("[#flag-attr]").unwrap();

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(document);

    assert!(!context.flag_export);
}

#[test]
fn evaluate_variadic() {
    let mut context = CustomContext::default();

    let document = parser::note("[#variadic first | second | third]").unwrap();

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(document);

    assert_eq!(
        context.variadic_values,
        vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string()
        ]
    );
}
