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
        self.values.iter().find(|x| x.key == key).is_some()
    }

    pub fn get_value(&self, key: &str) -> Option<&'input str> {
        self.values
            .iter()
            .find(|x| x.key == key)
            .and_then(|x| x.value)
    }
}
