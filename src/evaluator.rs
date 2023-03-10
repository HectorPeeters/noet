use crate::{
    context::Context,
    parse_tree::{Block, ParsedAttribute, ParsedElement},
    registry::FunctionRegistry,
    value::Value,
};

pub struct Evaluator<'context, Context, Value> {
    pub context: &'context mut Context,
    function_registry: FunctionRegistry<Context, Value>,
}

impl<'context, 'input, C, V> Evaluator<'context, C, V>
where
    C: Context<V>,
    V: Value<'input>,
{
    pub fn new(context: &'context mut C) -> Self {
        let mut function_registry = FunctionRegistry::new();
        C::register_functions(&mut function_registry);

        Self {
            context,
            function_registry,
        }
    }

    fn evaluate_block(&mut self, block: Block<'input>) -> Vec<V> {
        block
            .elements
            .into_iter()
            .filter_map(|e| self.evaluate_element(e))
            .collect()
    }

    fn evaluate_function(
        &mut self,
        name: &'input str,
        attributes: Vec<ParsedAttribute<'input>>,
        arguments: Vec<Block<'input>>,
    ) -> Option<V> {
        let evaluated_arguments = arguments
            .into_iter()
            .map(|a| self.evaluate_block(a))
            .collect::<Vec<_>>();

        match self.function_registry.get(name) {
            Some(func) => func(self.context, &vec![], &evaluated_arguments),
            None => panic!("Function '{name}' not found"),
        }
    }

    fn evaluate_element(&mut self, element: ParsedElement<'input>) -> Option<V> {
        match element {
            ParsedElement::Function(name, attributes, arguments) => {
                self.evaluate_function(name, attributes, arguments)
            }
            _ => Some(V::from(element)),
        }
    }

    pub fn evaluate_document<I>(&mut self, document: I) -> Vec<V>
    where
        I: Iterator<Item = ParsedElement<'input>>,
    {
        document.filter_map(|e| self.evaluate_element(e)).collect()
    }
}
