use crate::argument::Argument;

pub struct Attribute<const NAME: &'static str, T> {
    inner: Option<T>,
}

impl<const NAME: &'static str, T> Attribute<NAME, T> {
    pub fn into_inner(self) -> Option<T> {
        self.inner
    }
}

impl<'a, const NAME: &'static str, T, Value: 'a> Argument<'a, Value> for Attribute<NAME, T>
where
    T: Argument<'a, Value>,
{
    fn from_value(value: &'a Value) -> Option<Self> {
        Argument::from_value(value).map(|inner| Self { inner })
    }

    fn from_values<I>(_values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a Value>,
    {
        unreachable!()
    }

    fn from_attributes<I>(attributes: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a (&'a str, Option<Value>)>,
    {
        attributes
            .find(|(n, _)| n == &NAME)
            .map(|(_, x)| Argument::from_value(x.as_ref().unwrap()))
            .map(|inner| Attribute { inner })
    }

    fn is_attribute() -> bool {
        true
    }
}
