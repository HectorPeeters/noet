use crate::value::Value;

pub trait Argument<'input, Value: 'input>
where
    Self: Sized,
{
    fn from_block(value: &'input Vec<Value>) -> Option<Self>;

    fn from_blocks<I>(values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'input Vec<Value>>,
    {
        values.next().and_then(Self::from_block)
    }
}

impl<'input, V: 'input> Argument<'input, V> for V
where
    V: Value<'input>,
{
    fn from_block(value: &'input Vec<V>) -> Option<Self> {
        assert_eq!(value.len(), 1);

        let first = &value[0];
        Some(first.clone())
    }
}

impl<'input, V: 'input> Argument<'input, V> for Vec<V>
where
    V: Value<'input>,
{
    fn from_block(value: &'input Vec<V>) -> Option<Self> {
        assert_eq!(value.len(), 1);
        Some(value.clone())
    }
}
