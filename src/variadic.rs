use crate::{argument::Argument, error::Result};

pub struct Variadic<T> {
    inner: Vec<T>,
}

impl<T> From<Variadic<T>> for Vec<T> {
    fn from(val: Variadic<T>) -> Self {
        val.inner
    }
}

impl<'input, Value: 'input, T> Argument<'input, Value> for Variadic<T>
where
    T: Argument<'input, Value>,
{
    fn from_value(value: Value) -> Result<Self> {
        Ok(Variadic {
            inner: vec![T::from_value(value)?],
        })
    }

    fn from_values<I>(values: &mut I) -> Result<Self>
    where
        I: Iterator<Item = Value>,
    {
        Ok(Variadic {
            inner: values
                .map(|v| T::from_value(v))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}
