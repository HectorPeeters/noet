use crate::parse_tree::ParsedElement;

pub trait Value<'input>
where
    Self: Sized + Clone + From<ParsedElement<'input>>,
{
}
