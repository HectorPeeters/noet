pub trait Value<'input>
where
    Self: Sized,
{
    fn from_text(text: &'input str) -> Option<Self>;
    fn from_pagebreak() -> Option<Self>;
}
