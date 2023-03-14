use std::any::type_name;

use crate::{
    context::Context,
    error::{Error, Result},
    evaluator::Evaluator,
    parse_tree::ParsedElement,
    value::Value,
};

pub trait Argument<'input, C, V>
where
    Self: Sized,
{
    fn from_element(
        evaluator: &Evaluator<C, V>,
        context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Self>;

    fn from_elements<I>(
        evaluator: &Evaluator<C, V>,
        context: &mut C,
        elements: &mut I,
    ) -> Result<Self>
    where
        I: Iterator<Item = ParsedElement<'input>>,
    {
        elements
            .next()
            .map(|v| Self::from_element(evaluator, context, v))
            .ok_or(Error::Type(
                format!(
                    "Argument of type {} is missing a value",
                    type_name::<Self>()
                ),
                None,
            ))?
    }
}

impl<'input, C, V> Argument<'input, C, V> for V
where
    V: Value<'input>,
    C: Context<V>,
{
    fn from_element(
        evaluator: &Evaluator<C, V>,
        context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Self> {
        evaluator
            .evaluate_element(context, element)?
            .ok_or(Error::Eval(
                format!(
                    "Evaluating argument of type {} did not result in a value",
                    type_name::<V>()
                ),
                None,
            ))
    }
}

impl<'input, C, V> Argument<'input, C, V> for ParsedElement<'input> {
    fn from_element(
        _evaluator: &Evaluator<C, V>,
        _context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Self> {
        Ok(element)
    }
}

impl<'input, C, V> Argument<'input, C, V> for String {
    fn from_element(
        _evaluator: &Evaluator<C, V>,
        _context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Self> {
        match element {
            ParsedElement::Text(text) => Ok(text.to_string()),
            _ => panic!(),
        }
    }
}

impl<'input, C, V> Argument<'input, C, V> for &'input str {
    fn from_element(
        _evaluator: &Evaluator<C, V>,
        _context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Self> {
        match element {
            ParsedElement::Text(text) => Ok(text),
            _ => panic!(),
        }
    }
}

// NOTE: A generic implementation using the FromStr trait wouldn't work here as we also implement
// the Argument trait for the generic Value type. This leads to duplicate implementations in cases
// where a Value type also implements FromStr.
macro_rules! impl_primitive_type {
    ($typ:ty) => {
        impl<'context, 'input, C, V> Argument<'input, C, V> for $typ {
            fn from_element(
                _evaluator: &Evaluator<C, V>,
                _context: &mut C,
                element: ParsedElement<'input>,
            ) -> Result<Self> {
                match element {
                    ParsedElement::Text(text) => text.parse().map_err(|_| {
                        Error::Type(
                            format!(
                                "Failed to convert '{text}' to {} in argument",
                                type_name::<$typ>()
                            ),
                            None,
                        )
                    }),
                    _ => panic!(),
                }
            }
        }
    };
}

impl_primitive_type!(bool);

impl_primitive_type!(u8);
impl_primitive_type!(u16);
impl_primitive_type!(u32);
impl_primitive_type!(u64);

impl_primitive_type!(i8);
impl_primitive_type!(i16);
impl_primitive_type!(i32);
impl_primitive_type!(i64);
