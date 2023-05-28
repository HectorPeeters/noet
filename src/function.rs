use crate::{
    argument::Argument, attribute::Attrs, error::Result, evaluator::Evaluator,
    parse_tree::ParsedElement, return_value::ReturnValue,
};

pub type Function<Context, Value> = Box<
    dyn Fn(
        &Evaluator<Context, Value>,
        &mut Context,
        Attrs,
        Vec<ParsedElement>,
    ) -> Result<Option<Value>>,
>;

pub trait ToFunction<'context, Context, Value, Args, Return> {
    fn to_function(self) -> Function<Context, Value>;
}

impl<'context, A, Context, Value, Func, Return> ToFunction<'context, Context, Value, (A,), Return>
    for Func
where
    A: for<'a> Argument<'a, Context, Value>,
    Func: Fn(&mut Context, Attrs, A) -> Return + 'static,
    Return: ReturnValue<Value>,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |evaluator, context, attrs, args| {
            let mut args = args.into_iter();

            let arg = A::from_elements(evaluator, context, &mut args)?;

            let result = self(context, attrs, arg);
            Return::to_result_of_option(result)
        })
    }
}

impl<'context, A, B, Context, Value, Func, Return>
    ToFunction<'context, Context, Value, (A, B), Return> for Func
where
    A: for<'a> Argument<'a, Context, Value>,
    B: for<'a> Argument<'a, Context, Value>,
    Func: Fn(&mut Context, Attrs, A, B) -> Return + 'static,
    Return: ReturnValue<Value>,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |evaluator, context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_elements(evaluator, context, &mut args)?;
            let arg2 = B::from_elements(evaluator, context, &mut args)?;

            let result = self(context, attrs, arg1, arg2);
            Return::to_result_of_option(result)
        })
    }
}

impl<'context, A, B, C, Context, Value, Func, Return>
    ToFunction<'context, Context, Value, (A, B, C), Return> for Func
where
    A: for<'a> Argument<'a, Context, Value>,
    B: for<'a> Argument<'a, Context, Value>,
    C: for<'a> Argument<'a, Context, Value>,
    Func: Fn(&mut Context, Attrs, A, B, C) -> Return + 'static,
    Return: ReturnValue<Value>,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |evaluator, context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_elements(evaluator, context, &mut args)?;
            let arg2 = B::from_elements(evaluator, context, &mut args)?;
            let arg3 = C::from_elements(evaluator, context, &mut args)?;

            let result = self(context, attrs, arg1, arg2, arg3);
            Return::to_result_of_option(result)
        })
    }
}

impl<'context, A, B, C, D, Context, Value, Func, Return>
    ToFunction<'context, Context, Value, (A, B, C, D), Return> for Func
where
    A: for<'a> Argument<'a, Context, Value>,
    B: for<'a> Argument<'a, Context, Value>,
    C: for<'a> Argument<'a, Context, Value>,
    D: for<'a> Argument<'a, Context, Value>,
    Func: Fn(&mut Context, Attrs, A, B, C, D) -> Return + 'static,
    Return: ReturnValue<Value>,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |evaluator, context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_elements(evaluator, context, &mut args)?;
            let arg2 = B::from_elements(evaluator, context, &mut args)?;
            let arg3 = C::from_elements(evaluator, context, &mut args)?;
            let arg4 = D::from_elements(evaluator, context, &mut args)?;

            let result = self(context, attrs, arg1, arg2, arg3, arg4);
            Return::to_result_of_option(result)
        })
    }
}

impl<'context, A, B, C, D, E, Context, Value, Func, Return>
    ToFunction<'context, Context, Value, (A, B, C, D, E), Return> for Func
where
    A: for<'a> Argument<'a, Context, Value>,
    B: for<'a> Argument<'a, Context, Value>,
    C: for<'a> Argument<'a, Context, Value>,
    D: for<'a> Argument<'a, Context, Value>,
    E: for<'a> Argument<'a, Context, Value>,
    Func: Fn(&mut Context, Attrs, A, B, C, D, E) -> Return + 'static,
    Return: ReturnValue<Value>,
{
    fn to_function(self) -> Function<Context, Value> {
        Box::new(move |evaluator, context, attrs, args| {
            let mut args = args.into_iter();

            let arg1 = A::from_elements(evaluator, context, &mut args)?;
            let arg2 = B::from_elements(evaluator, context, &mut args)?;
            let arg3 = C::from_elements(evaluator, context, &mut args)?;
            let arg4 = D::from_elements(evaluator, context, &mut args)?;
            let arg5 = E::from_elements(evaluator, context, &mut args)?;

            let result = self(context, attrs, arg1, arg2, arg3, arg4, arg5);
            Return::to_result_of_option(result)
        })
    }
}
