use crate::parse_tree::ParsedElement;

pub trait Value<'input>
where
    Self: Sized + From<ParsedElement<'input>>,
{
    const LINEBREAK: Option<Self>;

    fn from_text_element(text: &'input str) -> Option<Self>;
}
