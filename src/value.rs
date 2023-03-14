pub trait Value<'input>
where
    Self: Sized,
{
    const LINEBREAK: Option<Self>;

    fn from_text_element(text: &'input str) -> Option<Self>;

    fn from_block_element(elements: Vec<Self>) -> Option<Self>;
}

pub struct EmptyValue {}

impl<'input> Value<'input> for EmptyValue {
    const LINEBREAK: Option<Self> = None;

    fn from_text_element(_text: &'input str) -> Option<Self> {
        None
    }

    fn from_block_element(_elements: Vec<Self>) -> Option<Self> {
        None
    }
}
