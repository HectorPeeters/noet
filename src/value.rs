use crate::parse_tree::ParsedElement;

pub type BlockValue<Value> = Vec<Value>;

pub trait Value<'input>
where
    Self: Sized + Clone + From<ParsedElement<'input>>,
{
}
