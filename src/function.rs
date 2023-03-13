use crate::{argument::Argument, attribute::Attrs};

pub type Function<Context, Value> = Box<dyn Fn(&mut Context, Attrs, Vec<Value>) -> Option<Value>>;

pub trait ToFunction<Context, Value, Args> {
    fn to_function(self) -> Function<Context, Value>;
}

impl<A, Context, Value, Func> ToFunction<Context, Value, (A,)> for Func
where
    A: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, Attrs, A) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg = A::from_values(&mut args).unwrap();

            self(context, attrs, arg)
        })
    }
}

impl<A, B, Context, Value, Func> ToFunction<Context, Value, (A, B)> for Func
where
    A: for<'a> Argument<'a, Value>,
    B: for<'a> Argument<'a, Value>,
    Func: Fn(&mut Context, Attrs, A, B) -> Option<Value> + 'static,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |context, attrs, args| {
            let mut args = args.iter();

            // TODO: get rid of unwrap
            let arg1 = A::from_values(&mut args).unwrap();
            let arg2 = B::from_values(&mut args).unwrap();

            self(context, attrs, arg1, arg2)
        })
    }
}
