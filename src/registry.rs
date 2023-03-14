use std::collections::HashMap;

use crate::function::{Function, ToFunction};

#[derive(Default)]
pub struct FunctionRegistry<Context, Value> {
    bindings: HashMap<&'static str, Function<Context, Value>>,
}

impl<Context, Value> FunctionRegistry<Context, Value> {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn register_function<F, A, R>(&mut self, func: F, name: &'static str)
    where
        F: for<'a> ToFunction<'a, Context, Value, A, R>,
    {
        self.bindings.insert(name, func.to_function());
    }

    pub fn get(&self, name: &str) -> Option<&Function<Context, Value>> {
        self.bindings.get(name)
    }
}
