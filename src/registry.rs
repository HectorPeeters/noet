use std::collections::HashMap;

use crate::function::{Function, ToFunction};

pub struct FunctionRegistry<Context, Value> {
    bindings: HashMap<&'static str, Function<Context, Value>>,
}

impl<Context, Value> FunctionRegistry<Context, Value> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn register_function<F, A>(&mut self, func: F, name: &'static str)
    where
        F: ToFunction<Context, Value, A>,
    {
        self.bindings.insert(name, func.to_function());
    }
}

impl<Context, Value> Default for FunctionRegistry<Context, Value> {
    fn default() -> Self {
        Self::new()
    }
}
