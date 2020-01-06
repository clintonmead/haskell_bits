use crate::*;

pub struct TypeCon;

impl<T> TypeAppParam for Option<T> {
    type Param = T;
}

impl<T> TypeApp<TypeCon, T> for Option<T> {}

impl<T> WithTypeArg<T> for TypeCon {
    type Type = Option<T>;
}

impl Functor for TypeCon {
    fn fmap<TIn, TOut>(
        f: impl Fn(&TIn) -> TOut,
        x: &<TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type {
        Option::map(x.as_ref(), f)
    }
}

impl LinearFunctor for TypeCon {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type {
        Option::map(x, f)
    }
}

impl Lift for TypeCon {
    fn lift<T>(x: T) -> <TypeCon as WithTypeArg<T>>::Type {
        From::from(x)
    }
}

impl Applicative for TypeCon {
    fn ap<TIn, TOut, TFunc>(
        f: &<TypeCon as WithTypeArg<TFunc>>::Type,
        x: &<TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn) -> TOut,
    {
        f.as_ref().and_then(|f_val| fmap(f_val, x))
    }

    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<Self as WithTypeArg<TIn1>>::Type,
        x2: &<Self as WithTypeArg<TIn2>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn1, &TIn2) -> TOut,
    {
        x1.as_ref()
            .and_then(|x1_val| x2.as_ref().map(|x2_val| f(x1_val, x2_val)))
    }
}

impl LinearApplicative for TypeCon {
    fn lap<TIn, TOut, TFunc>(
        f: <TypeCon as WithTypeArg<TFunc>>::Type,
        x: <TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: FnOnce(TIn) -> TOut,
    {
        f.and_then(|f_val| x.map(|x_val| f_val(x_val)))
    }

    fn llift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: <TypeCon as WithTypeArg<TIn1>>::Type,
        x2: <TypeCon as WithTypeArg<TIn2>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: FnOnce(TIn1, TIn2) -> TOut,
    {
        x1.and_then(|x1val| x2.map(|x2val| f(x1val, x2val)))
    }
}

impl Monad for TypeCon {
    fn bind<TIn, TOut, TFuncArg>(
        x: &<TypeCon as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        TFuncArg: Fn(&TIn) -> <TypeCon as WithTypeArg<TOut>>::Type,
    {
        x.as_ref().and_then(f)
    }
}
