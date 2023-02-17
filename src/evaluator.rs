use crate::{
    context::Context,
    parse_tree::{Attribute, ParsedDocument, ParsedElement},
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
    ) -> Option<V> {
        let evaluated_arguments = arguments
            .into_iter()
            .filter_map(|a| self.evaluate_element(a))
            .collect::<Vec<_>>();

        //        let evaluated_attributes = attributes
        //            .iter()
        //            .filter_map(|attr| match attr {
        //                Attribute::Flag(name) => ,
        //                Attribute::Value(_, _) => todo!(),
        //            })
        //            .collect::<Vec<_>>();

        match self.function_registry.get(name) {
            Some(func) => func(self.context, &[], &evaluated_arguments),
            None => panic!("Function '{name}' not found"),
        }
    }

    fn evaluate_element(&mut self, element: ParsedElement<'input>) -> Option<V> {
        match element {
            ParsedElement::Text(t) => V::from_text(t),
            ParsedElement::Function(name, attributes, arguments) => {
                self.evaluate_function(name, attributes, arguments)
            }
            ParsedElement::ParagraphBreak() => V::from_pagebreak(),
        }
    }

    pub fn evaluate_document(&mut self, document: ParsedDocument<'input>) -> Vec<V> {
        document
            .elements
            .into_iter()
            .filter_map(|e| self.evaluate_element(e))
            .collect()
    }
}
