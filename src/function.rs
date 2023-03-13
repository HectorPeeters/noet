use crate::{
    argument::Argument, attribute::Attrs, error::Result, evaluator::Evaluator,
    parse_tree::ParsedElement,
};

pub type Function<Context, Value> = Box<
    dyn Fn(
        &Evaluator<Context, Value>,
        &mut Context,
        Attrs,
        Vec<ParsedElement>,
    ) -> Result<Option<Value>>,
>;

pub trait ToFunction<'context, Context, Value, Args> {
    fn to_function(self) -> Function<Context, Value>;
}

macro_rules! impl_func_args_1 {
    ($ret:ty, $ret_map:expr) => {
        impl<'context, A, Context, Value, Func> ToFunction<'context, Context, Value, (A, $ret)>
            for Func
        where
            A: for<'a> Argument<'a, Context, Value>,
            Func: Fn(&mut Context, Attrs, A) -> Result<$ret> + 'static,
        {
            fn to_function(self) -> Function<Context, Value> {
                Box::new(move |evaluator, context, attrs, args| {
                    let mut args = args.into_iter();

                    let arg = A::from_elements(evaluator, context, &mut args)?;

                    self(context, attrs, arg).map($ret_map)
                })
            }
        }
    };
}

impl_func_args_1!((), |_| None);
impl_func_args_1!(Option<Value>, |x| x);

macro_rules! impl_func_args_2 {
    ($ret:ty, $ret_map:expr) => {
        impl<'context, A, B, Context, Value, Func>
            ToFunction<'context, Context, Value, (A, B, $ret)> for Func
        where
            A: for<'a> Argument<'a, Context, Value>,
            B: for<'a> Argument<'a, Context, Value>,
            Func: Fn(&mut Context, Attrs, A, B) -> Result<$ret> + 'static,
        {
            fn to_function(self) -> Function<Context, Value> {
                Box::new(move |evaluator, context, attrs, args| {
                    let mut args = args.into_iter();

                    let arg1 = A::from_elements(evaluator, context, &mut args)?;
                    let arg2 = B::from_elements(evaluator, context, &mut args)?;

                    self(context, attrs, arg1, arg2).map($ret_map)
                })
            }
        }
    };
}

impl_func_args_2!((), |_| None);
impl_func_args_2!(Option<Value>, |x| x);

macro_rules! impl_func_args_3 {
    ($ret:ty, $ret_map:expr) => {
        impl<'context, A, B, C, Context, Value, Func>
            ToFunction<'context, Context, Value, (A, B, C, $ret)> for Func
        where
            A: for<'a> Argument<'a, Context, Value>,
            B: for<'a> Argument<'a, Context, Value>,
            C: for<'a> Argument<'a, Context, Value>,
            Func: Fn(&mut Context, Attrs, A, B, C) -> Result<$ret> + 'static,
        {
            fn to_function(self) -> Function<Context, Value> {
                Box::new(move |evaluator, context, attrs, args| {
                    let mut args = args.into_iter();

                    let arg1 = A::from_elements(evaluator, context, &mut args)?;
                    let arg2 = B::from_elements(evaluator, context, &mut args)?;
                    let arg3 = C::from_elements(evaluator, context, &mut args)?;

                    self(context, attrs, arg1, arg2, arg3).map($ret_map)
                })
            }
        }
    };
}

impl_func_args_3!((), |_| None);
impl_func_args_3!(Option<Value>, |x| x);

macro_rules! impl_func_args_4 {
    ($ret:ty, $ret_map:expr) => {
        impl<'context, A, B, C, D, Context, Value, Func>
            ToFunction<'context, Context, Value, (A, B, C, D, $ret)> for Func
        where
            A: for<'a> Argument<'a, Context, Value>,
            B: for<'a> Argument<'a, Context, Value>,
            C: for<'a> Argument<'a, Context, Value>,
            D: for<'a> Argument<'a, Context, Value>,
            Func: Fn(&mut Context, Attrs, A, B, C, D) -> Result<$ret> + 'static,
        {
            fn to_function(self) -> Function<Context, Value> {
                Box::new(move |evaluator, context, attrs, args| {
                    let mut args = args.into_iter();

                    let arg1 = A::from_elements(evaluator, context, &mut args)?;
                    let arg2 = B::from_elements(evaluator, context, &mut args)?;
                    let arg3 = C::from_elements(evaluator, context, &mut args)?;
                    let arg4 = D::from_elements(evaluator, context, &mut args)?;

                    self(context, attrs, arg1, arg2, arg3, arg4).map($ret_map)
                })
            }
        }
    };
}

impl_func_args_4!((), |_| None);
impl_func_args_4!(Option<Value>, |x| x);
