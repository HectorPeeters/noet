use toen::{
    argument::Argument, context::Context, evaluator::Evaluator, parse_tree::ParsedElement,
    parser::parser, registry::FunctionRegistry, value::Value,
};

pub enum CustomValue {
    Text(String),
}

impl<'input> Value<'input> for CustomValue {
    fn from_element(element: &ParsedElement<'input>) -> Option<Self> {
        match element {
            ParsedElement::Text(t) => Some(CustomValue::Text(t.to_string())),
            _ => None,
        }
    }
}

impl<'a> Argument<'a, CustomValue> for String {
    fn from_value(value: &'a CustomValue) -> Option<Self> {
        match value {
            CustomValue::Text(t) => Some(t.to_string()),
        }
    }
}

#[derive(Default)]
pub struct CustomContext {
    pub value: String,
}

impl Context<CustomValue> for CustomContext {
    fn register_functions(registry: &mut FunctionRegistry<Self, CustomValue>) {
        registry.register_function(func_test, "test");
    }
}

fn func_test(context: &mut CustomContext, value: String) -> Option<CustomValue> {
    context.value = value;
    None
}

#[test]
fn evaluate_single_function() {
    let mut context = CustomContext::default();

    let document = parser::note("[#test test]").unwrap();

    let mut evaluator = Evaluator::new(&mut context);
    evaluator.evaluate_document(document);

    assert_eq!(context.value, "test");
}
