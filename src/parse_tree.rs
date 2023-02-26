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

#[derive(Debug, PartialEq)]
pub enum ParsedElement<'input> {
    Text(&'input str),
    Function(&'input str, Vec<Attribute<'input>>, Vec<Block<'input>>),
    ParagraphBreak(),
}

#[derive(Debug, PartialEq)]
pub struct Block<'input> {
    pub elements: Vec<ParsedElement<'input>>,
}

impl<'input> Block<'input> {
    pub fn new(elements: Vec<ParsedElement<'input>>) -> Self {
        Self { elements }
    }
}
