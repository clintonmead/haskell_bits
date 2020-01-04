use crate::*;

pub struct VecTypeCon;

impl<T> TypeAppParam for Vec<T> {
    type Param = T;
}

impl<T> TypeApp<VecTypeCon, T> for Vec<T> {}

impl<T> WithTypeArg<T> for VecTypeCon {
    type Type = Vec<T>;
}

impl Functor for VecTypeCon {
    fn fmap<TIn, TOut>(
        f: impl Fn(&TIn) -> TOut,
        x: &<VecTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <VecTypeCon as WithTypeArg<TOut>>::Type {
        x.iter().map(f).collect()
    }
}

impl LinearFunctor for VecTypeCon {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <VecTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <VecTypeCon as WithTypeArg<TOut>>::Type {
        x.into_iter().map(f).collect()
    }
}

impl Lift for VecTypeCon {
    fn lift<T>(x: T) -> <VecTypeCon as WithTypeArg<T>>::Type {
        vec![x]
    }
}

impl Applicative for VecTypeCon {
    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<VecTypeCon as WithTypeArg<TIn1>>::Type,
        x2: &<VecTypeCon as WithTypeArg<TIn2>>::Type,
    ) -> <VecTypeCon as WithTypeArg<TOut>>::Type
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

impl Monad for VecTypeCon {
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
