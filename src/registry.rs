use std::collections::HashMap;

use crate::function::Function;

pub struct FunctionRegistry<Context, Value> {
    bindings: HashMap<&'static str, Function<Context, Value>>,
}

impl<Context, Value> FunctionRegistry<Context, Value> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }
}

impl<Context, Value> Default for FunctionRegistry<Context, Value> {
    fn default() -> Self {
        Self::new()
    }
}
