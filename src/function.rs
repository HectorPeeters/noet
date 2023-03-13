use crate::{argument::Argument, attribute::Attrs, error::Result};

pub type Function<Context, Value> =
    Box<dyn Fn(&mut Context, Attrs, Vec<Value>) -> Result<Option<Value>>>;

pub trait ToFunction<Context, Value, Args> {
    fn to_function(self) -> Function<Context, Value>;
}

impl<A, Context, Value, Func> ToFunction<Context, Value, (A,)> for Func
where
    A: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, Attrs, A) -> Result<Option<Value>> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut args = args.into_iter();

            let arg = A::from_values(&mut args)?;

            self(context, attrs, arg)
        })
    }
}

impl<A, B, Context, Value, Func> ToFunction<Context, Value, (A, B)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, Attrs, A, B) -> Result<Option<Value>> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_values(&mut args)?;
            let arg2 = B::from_values(&mut args)?;

            self(context, attrs, arg1, arg2)
        })
    }
}

impl<A, B, C, Context, Value, Func> ToFunction<Context, Value, (A, B, C)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    C: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, Attrs, A, B, C) -> Result<Option<Value>> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_values(&mut args)?;
            let arg2 = B::from_values(&mut args)?;
            let arg3 = C::from_values(&mut args)?;

            self(context, attrs, arg1, arg2, arg3)
        })
    }
}

impl<A, B, C, D, Context, Value, Func> ToFunction<Context, Value, (A, B, C, D)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    C: for<'a> Argument<'a, Value>,
    D: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, Attrs, A, B, C, D) -> Result<Option<Value>> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_values(&mut args)?;
            let arg2 = B::from_values(&mut args)?;
            let arg3 = C::from_values(&mut args)?;
            let arg4 = D::from_values(&mut args)?;

            self(context, attrs, arg1, arg2, arg3, arg4)
        })
    }
}
