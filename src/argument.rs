pub trait Argument<'a, Value: 'a>
where
    Self: Sized,
{
    fn from_value(value: &'a Value) -> Option<Self>;

    fn from_values<I>(values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a Value>,
    {
        match values.next() {
            Some(x) => Self::from_value(x),
            None => None,
        }
    }

    fn from_attributes<I>(_attributes: &mut I) -> Option<Self>
    where
        // TODO: (&'a str, Value) can be its own type
        I: Iterator<Item = &'a (&'a str, Value)>,
    {
        unreachable!()
    }

    fn is_attribute() -> bool {
        false
    }
}

impl<'a, Value: 'a> Argument<'a, Value> for () {
    fn from_value(_: &'a Value) -> Option<Self> {
        unreachable!()
    }
}

impl<'a, T, Value: 'a> Argument<'a, Value> for Option<T>
where
    T: Argument<'a, Value>,
{
    fn from_value(value: &'a Value) -> Option<Self> {
        Some(Argument::from_value(value))
    }

    fn from_values<I>(values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a Value>,
    {
        values.next().map(|x| Argument::from_value(x))
    }
}
