use std::any::type_name;

use crate::{
    error::{Error, Result},
    parse_tree::ParsedElement,
    value::Value,
};

pub trait Argument<'input>
where
    Self: Sized,
{
    fn from_element(element: ParsedElement<'input>) -> Result<Self>;

    fn from_elements<I>(elements: &mut I) -> Result<Self>
    where
        I: Iterator<Item = ParsedElement<'input>>,
    {
        elements
            .next()
            .map(|v| Self::from_element(v))
            .ok_or(Error::Type(
                format!(
                    "Argument of type {} is missing a value",
                    type_name::<Self>()
                ),
                None,
            ))?
    }
}

impl<'input, V> Argument<'input> for V
where
    V: Value<'input>,
{
    fn from_element(element: ParsedElement<'input>) -> Result<Self> {
        Ok(element.into())
    }
}

impl<'input> Argument<'input> for ParsedElement<'input> {
    fn from_element(element: ParsedElement<'input>) -> Result<Self> {
        Ok(element)
    }
}

impl<'input> Argument<'input> for String {
    fn from_element(element: ParsedElement<'input>) -> Result<Self> {
        match element {
            ParsedElement::Text(text) => Ok(text.to_string()),
            _ => panic!(),
        }
    }
}

impl<'input> Argument<'input> for &'input str {
    fn from_element(element: ParsedElement<'input>) -> Result<Self> {
        match element {
            ParsedElement::Text(text) => Ok(text),
            _ => panic!(),
        }
    }
}

macro_rules! impl_numeric {
    ($typ:ty) => {
        impl<'input> Argument<'input> for $typ {
            fn from_element(element: ParsedElement<'input>) -> Result<Self> {
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

impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);

impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
