use crate::{
    attribute::{Attribute, Attrs},
    context::Context,
    error::{Error, Result},
    parse_tree::ParsedElement,
    registry::FunctionRegistry,
    value::Value,
};

pub struct Evaluator<Context, Value> {
    function_registry: FunctionRegistry<Context, Value>,
}

impl<'input, C, V> Evaluator<C, V>
where
    C: Context<V>,
    V: Value<'input>,
{
    pub fn new() -> Self {
        let mut function_registry = FunctionRegistry::new();
        C::register_functions(&mut function_registry);

        Self { function_registry }
    }

    fn evaluate_function(
        &self,
        context: &mut C,
        name: &'input str,
        attributes: Vec<Attribute<'input>>,
        arguments: Vec<ParsedElement<'input>>,
    ) -> Result<Option<V>> {
        match self.function_registry.get(name) {
            Some(func) => func(self, context, Attrs::new(attributes), arguments),
            None => Err(Error::Eval(format!("Function '{name}' not found"), None)),
        }
    }

    pub fn evaluate_element(
        &self,
        context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Option<V>> {
        match element {
            ParsedElement::HardLinebreak() => Ok(V::LINEBREAK),
            ParsedElement::Text(t) => Ok(V::from_text_element(t)),
            ParsedElement::Function(name, attributes, arguments) => {
                self.evaluate_function(context, name, attributes, arguments)
            }
            _ => Ok(Some(element.into())),
        }
    }

    pub fn evaluate_document<I>(&self, context: &mut C, document: I) -> Result<Vec<V>>
    where
        I: Iterator<Item = Result<ParsedElement<'input>>>,
    {
        let mut evaluated_elements = vec![];
        for element in document {
            if let Some(evaluated_element) = self.evaluate_element(context, element?)? {
                evaluated_elements.push(evaluated_element);
            }
        }

        Ok(evaluated_elements)
    }
}
