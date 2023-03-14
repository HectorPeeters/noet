use std::{slice::Iter, vec::IntoIter};

use crate::{argument::Argument, error::Result, evaluator::Evaluator, parse_tree::ParsedElement};

pub struct Variadic<T> {
    inner: Vec<T>,
}

impl<T> From<Variadic<T>> for Vec<T> {
    fn from(val: Variadic<T>) -> Self {
        val.inner
    }
}

impl<'input, C, V, T> Argument<'input, C, V> for Variadic<T>
where
    T: Argument<'input, C, V>,
{
    fn from_element(
        evaluator: &Evaluator<C, V>,
        context: &mut C,
        element: ParsedElement<'input>,
    ) -> Result<Self> {
        Ok(Variadic {
            inner: vec![T::from_element(evaluator, context, element)?],
        })
    }

    fn from_elements<I>(
        evaluator: &Evaluator<C, V>,
        context: &mut C,
        elements: &mut I,
    ) -> Result<Self>
    where
        I: Iterator<Item = ParsedElement<'input>>,
    {
        Ok(Variadic {
            inner: elements
                .map(|v| T::from_element(evaluator, context, v))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl<T> Variadic<T> {
    pub fn inner(&self) -> &[T] {
        &self.inner
    }

    pub fn into_inner(self) -> Vec<T> {
        self.inner
    }
}
