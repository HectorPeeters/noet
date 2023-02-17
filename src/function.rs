use crate::argument::Argument;

// TODO: attributes should have an optional value. This allows us to remove the `empty` function in
// the Value trait
pub type Function<Context, Value> =
    Box<dyn Fn(&mut Context, &[(&'_ str, Value)], &[Value]) -> Option<Value>>;

pub trait ToFunction<Context, Value, Args> {
    fn to_function(self) -> Function<Context, Value>;
}

macro_rules! eval_attr_or_arg {
    ($attrs:ident, $args:ident, $t:ty) => {{
        if <$t>::is_attribute() {
            <$t>::from_attributes(&mut $attrs)
        } else {
            <$t>::from_values(&mut $args)
        }
    }};
}

impl<A, Context, Value, Func> ToFunction<Context, Value, A> for Func
where
    A: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, A) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut attrs = attrs.iter();
            let mut args = args.iter();
            // TODO: Option<A> should be replaced by Result<A>
            let arg = eval_attr_or_arg!(attrs, args, A).unwrap();
            assert!(args.next().is_none());

            self(context, arg)
        })
    }
}
