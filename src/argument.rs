use crate::value::Value;

pub trait Argument<'input, Value: 'input>
where
    Self: Sized,
{
    fn from_block(value: &Value) -> Self;

    fn from_blocks<I>(values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'input Value>,
    {
        values.next().map(|v| Self::from_block(v))
    }
}

impl<'input, V: 'input> Argument<'input, V> for V
where
    V: Value<'input> + Clone,
{
    fn from_block(value: &V) -> Self {
        value.clone()
    }
}
