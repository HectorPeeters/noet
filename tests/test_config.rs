use toen::{
    context::Context,
    evaluator::{self, Evaluator},
    parser::parser,
    registry::FunctionRegistry,
};

pub enum CustomValue {
    Text(String),
}

#[derive(Default)]
pub struct CustomContext {
    pub test_ran: bool,
}

impl Context<CustomValue> for CustomContext {
    fn register_functions(registry: &mut FunctionRegistry<Self, CustomValue>) {
        registry.register_function(func_test, "test");
    }
}

fn func_test(context: &mut CustomContext, _: ()) -> Option<CustomValue> {
    context.test_ran = true;
    None
}

#[test]
fn evaluate_single_function() {
    let mut context = CustomContext::default();

    let mut evaluator = Evaluator::new(&mut context);
    let document = parser::note("[#test]");

    assert!(context.test_ran);
}
