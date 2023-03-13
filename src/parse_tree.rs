use crate::attribute::Attribute;

#[derive(Debug, PartialEq)]
pub enum ParsedElement<'input> {
    Text(&'input str),
    Function(
        &'input str,
        Vec<Attribute<'input>>,
        Vec<ParsedElement<'input>>,
    ),
    HardLinebreak(),
    Block(Vec<ParsedElement<'input>>),
}
