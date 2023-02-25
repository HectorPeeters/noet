#[derive(Debug, PartialEq)]
pub enum ParsedElement<'input> {
    Text(&'input str),
    Function(&'input str, Vec<Block<'input>>),
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
