use std::any::type_name;

use crate::{
    error::{Error, Result},
    value::Value,
};

pub trait Argument<'input, Value: 'input>
where
    Self: Sized,
{
    fn from_value(value: Value) -> Result<Self>;

    fn from_values<I>(values: &mut I) -> Result<Self>
    where
        I: Iterator<Item = Value>,
    {
        values
            .next()
            .map(|v| Self::from_value(v))
            .ok_or(Error::Type(
                format!(
                    "Argument of type {} is missing a value",
                    type_name::<Self>()
                ),
                None,
            ))?
    }
}

impl<'input, V: 'input> Argument<'input, V> for V
where
    V: Value<'input>,
{
    fn from_value(value: V) -> Result<Self> {
        Ok(value)
    }
}
