use crate::{argument::Argument, error::Result, parse_tree::ParsedElement};

pub struct Variadic<T> {
    inner: Vec<T>,
}

impl<T> From<Variadic<T>> for Vec<T> {
    fn from(val: Variadic<T>) -> Self {
        val.inner
    }
}

impl<'input, T> Argument<'input> for Variadic<T>
where
    T: Argument<'input>,
{
    fn from_element(element: ParsedElement<'input>) -> Result<Self> {
        Ok(Variadic {
            inner: vec![T::from_element(element)?],
        })
    }

    fn from_elements<I>(elements: &mut I) -> Result<Self>
    where
        I: Iterator<Item = ParsedElement<'input>>,
    {
        Ok(Variadic {
            inner: elements
                .map(|v| T::from_element(v))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}
