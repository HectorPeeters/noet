use crate::{
    attribute::{Attribute, Attrs},
    context::Context,
    error::Result,
    parse_tree::ParsedElement,
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

    fn evaluate_function(
        &mut self,
        name: &'input str,
        attributes: Vec<Attribute<'input>>,
        arguments: Vec<ParsedElement<'input>>,
    ) -> Result<Option<V>> {
        let mut evaluated_arguments = vec![];
        for arg in arguments {
            if let Some(evaluated_arg) = self.evaluate_element(arg)? {
                evaluated_arguments.push(evaluated_arg);
            }
        }

        let attrs = Attrs::new(attributes);

        match self.function_registry.get(name) {
            Some(func) => Ok(func(self.context, attrs, evaluated_arguments)),
            None => panic!("Function '{name}' not found"),
        }
    }

    fn evaluate_element(&mut self, element: ParsedElement<'input>) -> Result<Option<V>> {
        match element {
            ParsedElement::Function(name, attributes, arguments) => {
                self.evaluate_function(name, attributes, arguments)
            }
            _ => Ok(Some(V::from(element))),
        }
    }

    pub fn evaluate_document<I>(&mut self, document: I) -> Result<Vec<V>>
    where
        I: Iterator<Item = Result<ParsedElement<'input>>>,
    {
        let mut evaluated_elements = vec![];
        for element in document {
            if let Some(evaluated_element) = self.evaluate_element(element?)? {
                evaluated_elements.push(evaluated_element);
            }
        }

        Ok(evaluated_elements)
    }
}
