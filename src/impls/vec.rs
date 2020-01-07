use crate::*;

pub struct TypeCon;

impl<T> TypeAppParam for Vec<T> {
    type Param = T;
}

impl<T> TypeApp<TypeCon, T> for Vec<T> {}

impl<T> WithTypeArg<T> for TypeCon {
    type Type = Vec<T>;
}

impl Functor for TypeCon {
    fn fmap<TIn, TOut>(
        f: impl Fn(&TIn) -> TOut,
        x: &<TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type {
        x.iter().map(f).collect()
    }
}

impl LinearFunctor for TypeCon {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type {
        x.into_iter().map(f).collect()
    }
}

impl Lift for TypeCon {
    fn lift<T>(x: T) -> <TypeCon as WithTypeArg<T>>::Type {
        vec![x]
    }
}

impl Applicative for TypeCon {
    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<TypeCon as WithTypeArg<TIn1>>::Type,
        x2: &<TypeCon as WithTypeArg<TIn2>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn1, &TIn2) -> TOut,
    {
        x1.iter()
            .flat_map(|x1_val| {
                x2.iter()
                    .map(|x2_val| f(x1_val, x2_val))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl Monad for TypeCon {
    fn bind<TIn, TOut, TFuncArg>(
        x: &<Self as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        TFuncArg: Fn(&TIn) -> <Self as WithTypeArg<TOut>>::Type,
    {
        x.iter().flat_map(f).collect()
    }
}

impl LinearMonad for TypeCon {
    fn lbind<TIn, TOut, TFuncArg>(
        x: <Self as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        TFuncArg: Fn(TIn) -> <Self as WithTypeArg<TOut>>::Type,
    {
        x.into_iter().flat_map(f).collect()
    }
}
