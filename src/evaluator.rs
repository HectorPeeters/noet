use std::collections::HashMap;

use crate::function::Function;

pub struct Evaluator<'context, Context, Value> {
    pub context: &'context mut Context,
    bindings: HashMap<&'static str, Function<Context, Value>>,
}
