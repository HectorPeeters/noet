#[derive(Debug, PartialEq)]
pub enum ParsedAttribute<'input> {
    Flag(&'input str),
    Value(&'input str, ParsedElement<'input>),
}

#[derive(Debug, PartialEq)]
pub enum ParsedElement<'input> {
    Text(&'input str),
    Function(
        &'input str,
        Vec<ParsedAttribute<'input>>,
        Vec<ParsedElement<'input>>,
    ),
    ParagraphBreak(),
}

#[derive(Debug, PartialEq)]
pub struct ParsedDocument<'input> {
    pub elements: Vec<ParsedElement<'input>>,
}

impl<'input> ParsedDocument<'input> {
    pub fn new(elements: Vec<ParsedElement<'input>>) -> Self {
        Self { elements }
    }
}
