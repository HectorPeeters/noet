use crate::parse_tree::ParsedElement;

pub trait Value<'input>
where
    Self: Sized,
{
    fn from_element(element: &ParsedElement<'input>) -> Option<Self>;
}
