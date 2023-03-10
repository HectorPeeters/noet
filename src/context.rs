use crate::registry::FunctionRegistry;

pub trait Context<Value>
where
    Self: Sized,
{
    fn register_functions(registry: &mut FunctionRegistry<Self, Value>);
}
