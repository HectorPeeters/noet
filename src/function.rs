use crate::{
    argument::Argument,
    value::{AttributeValue, BlockValue},
};

pub type Function<Context, Value> =
    Box<dyn Fn(&mut Context, &[AttributeValue<Value>], &[BlockValue<Value>]) -> Option<Value>>;

pub trait ToFunction<Context, Value, Args> {
    fn to_function(self) -> Function<Context, Value>;
}

macro_rules! eval_attr_or_arg {
    ($attrs:ident, $args:ident, $t:ty) => {{
        if <$t>::IS_ATTRIBUTE {
            <$t>::from_attributes(&mut $attrs)
        } else {
            <$t>::from_blocks(&mut $args)
        }
    }};
}

impl<A, Context, Value, Func> ToFunction<Context, Value, (A,)> for Func
where
    A: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, A) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut attrs = attrs.iter();
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg = eval_attr_or_arg!(attrs, args, A).unwrap();

            self(context, arg)
        })
    }
}

impl<A, B, Context, Value, Func> ToFunction<Context, Value, (A, B)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, A, B) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut attrs = attrs.iter();
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg1 = eval_attr_or_arg!(attrs, args, A).unwrap();
            let arg2 = eval_attr_or_arg!(attrs, args, B).unwrap();

            self(context, arg1, arg2)
        })
    }
}

impl<A, B, C, Context, Value, Func> ToFunction<Context, Value, (A, B, C)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    C: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, A, B, C) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut attrs = attrs.iter();
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg1 = eval_attr_or_arg!(attrs, args, A).unwrap();
            let arg2 = eval_attr_or_arg!(attrs, args, B).unwrap();
            let arg3 = eval_attr_or_arg!(attrs, args, C).unwrap();

            self(context, arg1, arg2, arg3)
        })
    }
}

impl<A, B, C, D, Context, Value, Func> ToFunction<Context, Value, (A, B, C, D)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    C: for<'a> Argument<'a, Value>,
    D: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, A, B, C, D) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut attrs = attrs.iter();
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg1 = eval_attr_or_arg!(attrs, args, A).unwrap();
            let arg2 = eval_attr_or_arg!(attrs, args, B).unwrap();
            let arg3 = eval_attr_or_arg!(attrs, args, C).unwrap();
            let arg4 = eval_attr_or_arg!(attrs, args, D).unwrap();

            self(context, arg1, arg2, arg3, arg4)
        })
    }
}

impl<A, B, C, D, E, Context, Value, Func> ToFunction<Context, Value, (A, B, C, D, E)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    C: for<'a> Argument<'a, Value>,
    D: for<'a> Argument<'a, Value>,
    E: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, A, B, C, D, E) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut attrs = attrs.iter();
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg1 = eval_attr_or_arg!(attrs, args, A).unwrap();
            let arg2 = eval_attr_or_arg!(attrs, args, B).unwrap();
            let arg3 = eval_attr_or_arg!(attrs, args, C).unwrap();
            let arg4 = eval_attr_or_arg!(attrs, args, D).unwrap();
            let arg5 = eval_attr_or_arg!(attrs, args, E).unwrap();

            self(context, arg1, arg2, arg3, arg4, arg5)
        })
    }
}
