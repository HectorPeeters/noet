pub trait Value<'input>
where
    Self: Sized,
{
    const LINEBREAK: Option<Self>;

    fn from_text_element(text: &'input str) -> Option<Self>;

    fn from_block_element(elements: Vec<Self>) -> Option<Self>;
}
