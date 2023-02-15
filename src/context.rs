use crate::registry::FunctionRegistry;

pub trait Context<Value> {
    // TODO: replace with register_functions which takes a &mut FunctionRegistry
    fn register_functions(registry: &mut FunctionRegistry);
}
