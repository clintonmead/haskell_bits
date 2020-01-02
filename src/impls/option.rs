use crate::*;

pub struct OptionTypeCon;

impl<T> TypeAppParam for Option<T> {
    type Param = T;
}

impl<T> TypeApp<OptionTypeCon, T> for Option<T> {}

impl<T> WithTypeArg<T> for OptionTypeCon {
    type Type = Option<T>;
}

impl Functor for OptionTypeCon {
    fn fmap<TIn, TOut>(
        f: impl Fn(&TIn) -> TOut,
        x: &<OptionTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type {
        match x {
            None => None,
            Some(y) => Some(f(y)),
        }
    }
}

impl LinearFunctor for OptionTypeCon {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <OptionTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type {
        x.map(f)
    }
}

impl Lift for OptionTypeCon {
    fn lift<T>(x: T) -> <OptionTypeCon as WithTypeArg<T>>::Type {
        From::from(x)
    }
}

impl Applicative for OptionTypeCon {
    fn ap<TIn, TOut, TFunc>(
        f: &<OptionTypeCon as WithTypeArg<TFunc>>::Type,
        x: &<OptionTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn) -> TOut,
    {
        match f {
            None => None,
            Some(f_val) => fmap(f_val, x),
        }
    }

    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<Self as WithTypeArg<TIn1>>::Type,
        x2: &<Self as WithTypeArg<TIn2>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn1, &TIn2) -> TOut,
    {
        match x1 {
            None => None,
            Some(x1_val) => match x2 {
                None => None,
                Some(x2_val) => Some(f(x1_val, x2_val)),
            },
        }
    }
}

impl LinearApplicative for OptionTypeCon {
    fn lap<TIn, TOut, TFunc>(
        f: <OptionTypeCon as WithTypeArg<TFunc>>::Type,
        x: <OptionTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: FnOnce(TIn) -> TOut,
    {
        match f {
            None => None,
            Some(f_val) => match x {
                None => None,
                Some(x_val) => Some(f_val(x_val))
            }
        }
    }

    fn llift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: <OptionTypeCon as WithTypeArg<TIn1>>::Type,
        x2: <OptionTypeCon as WithTypeArg<TIn2>>::Type,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: FnOnce(TIn1, TIn2) -> TOut,
    {
        match x1 {
            None => None,
            Some(x1val) => match x2 {
                None => None,
                Some(x2val) => Some(f(x1val, x2val)),
            },
        }
    }
}

impl Monad for OptionTypeCon {
    fn bind<TIn, TOut, TFuncArg>(
        x: &<OptionTypeCon as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type
    where
        TFuncArg: Fn(&TIn) -> <OptionTypeCon as WithTypeArg<TOut>>::Type
        {
            match x {
                None => None,
                Some(x_val) => f(x_val)
            }
        }
}