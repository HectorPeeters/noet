use std::{any::type_name, fmt::Debug, str::FromStr};

use crate::error::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Attribute<'input> {
    pub key: &'input str,
    pub value: Option<&'input str>,
}

impl<'input> Attribute<'input> {
    pub fn new_flag(key: &'input str) -> Self {
        Self { key, value: None }
    }

    pub fn new_value(key: &'input str, value: &'input str) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }
}

pub struct Attrs<'input> {
    values: Vec<Attribute<'input>>,
}

impl<'input> Attrs<'input> {
    pub fn new(values: Vec<Attribute<'input>>) -> Self {
        Self { values }
    }

    pub fn has_flag(&self, key: &str) -> bool {
        self.values
            .iter()
            .any(|x| x.key == key && x.value.is_none())
    }

    pub fn get_value<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: FromStr,
        T::Err: Debug,
    {
        self.values
            .iter()
            .find(|x| x.key == key)
            .and_then(|x| x.value)
            .map(|x| {
                x.parse().map_err(|_| {
                    Error::Type(
                        format!(
                            "Failed to convert attribute value '{}' to {}",
                            x,
                            type_name::<T>()
                        ),
                        None,
                    )
                })
            })
            .transpose()
    }
}
