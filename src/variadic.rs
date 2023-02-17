use std::ops::{Deref, DerefMut};

use crate::argument::Argument;

pub struct Variadic<T> {
    values: Vec<T>,
}

impl<'a, Value: 'a, T> Argument<'a, Value> for Variadic<T>
where
    T: Argument<'a, Value>,
{
    fn from_value(value: &'a Value) -> Option<Self> {
        let elements = match Argument::from_value(value) {
            Some(element) => vec![element],
            None => Vec::new(),
        };

        Some(Self { values: elements })
    }

    fn from_values<I>(values: &mut I) -> Option<Self>
    where
        I: Iterator<Item = &'a Value>,
    {
        Some(Self {
            values: values.filter_map(|x| Argument::from_value(x)).collect(),
        })
    }
}

impl<T> Variadic<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.values
    }
}

impl<T> Iterator for Variadic<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop()
    }
}

impl<T> Deref for Variadic<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<T> DerefMut for Variadic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.values
    }
}
