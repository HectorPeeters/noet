use crate::{error::Result, value::Value};

pub trait ReturnValue<V> {
    fn to_result_of_option(self) -> Result<Option<V>>;
}

impl<'input, V> ReturnValue<V> for Result<Option<V>>
where
    V: Value<'input>,
{
    fn to_result_of_option(self) -> Result<Option<V>> {
        self
    }
}

impl<'input, V> ReturnValue<V> for Result<()>
where
    V: Value<'input>,
{
    fn to_result_of_option(self) -> Result<Option<V>> {
        self.map(|_| None)
    }
}

impl<'input, V> ReturnValue<V> for Result<V>
where
    V: Value<'input>,
{
    fn to_result_of_option(self) -> Result<Option<V>> {
        self.map(Some)
    }
}

impl<'input, V> ReturnValue<V> for Option<V>
where
    V: Value<'input>,
{
    fn to_result_of_option(self) -> Result<Option<V>> {
        Ok(self)
    }
}

impl<'input, V> ReturnValue<V> for V
where
    V: Value<'input>,
{
    fn to_result_of_option(self) -> Result<Option<V>> {
        Ok(Some(self))
    }
}

impl<'input, V> ReturnValue<V> for ()
where
    V: Value<'input>,
{
    fn to_result_of_option(self) -> Result<Option<V>> {
        Ok(None)
    }
}
