use crate::argument::Argument;

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
    fn from_block(value: &'input [Value]) -> Option<Self> {
        T::from_block(value).map(|v| Variadic { inner: vec![v] })
    }

    fn from_blocks<I>(values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'input Vec<Value>>,
    {
        Some(Variadic {
            inner: values.filter_map(|v| T::from_block(v)).collect(),
        })
    }
}
