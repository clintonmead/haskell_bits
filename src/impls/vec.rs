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
    fn fmap<TIn, TOut, F>(
        f: F,
        x: &<TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        F: Fn(&TIn) -> TOut,
    {
        x.iter().map(f).collect()
    }
}

impl LinearFunctor for TypeCon {
    fn lmap<TIn, TOut, F>(
        f: F,
        x: <TypeCon as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon as WithTypeArg<TOut>>::Type
    where
        F: Fn(TIn) -> TOut,
    {
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
    fn bind<TIn, TOut, F>(
        x: &<Self as WithTypeArg<TIn>>::Type,
        f: F,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        F: Fn(&TIn) -> <Self as WithTypeArg<TOut>>::Type,
    {
        x.iter().flat_map(f).collect()
    }
}

impl LinearFoldable for TypeCon {
    fn lfoldr<F, TIn, TOut>(f: F, init: TOut, x: <Self as WithTypeArg<TIn>>::Type) -> TOut
    where
        F: Fn(TIn, TOut) -> TOut,
    {
        x.into_iter().fold(init, |acc, next_val| f(next_val, acc))
    }
}

impl Foldable for TypeCon {
    fn foldr<F, TIn, TOut>(f: F, init: TOut, x: &<Self as WithTypeArg<TIn>>::Type) -> TOut
    where
        F: Fn(&TIn, TOut) -> TOut,
    {
        x.into_iter().fold(init, |acc, next_val| f(next_val, acc))
    }
}
/*
impl LinearTraversable for TypeCon {
    fn sequence<TApplicative, T>(
        x: <Self as WithTypeArg<<TApplicative as WithTypeArg<T>>::Type>>::Type,
    ) -> <TApplicative as WithTypeArg<<Self as WithTypeArg<T>>::Type>>::Type
    where
        TApplicative:
            LinearApplicative + WithTypeArg<T> + WithTypeArg<<Self as WithTypeArg<T>>::Type>,
        Self: WithTypeArg<T> + WithTypeArg<<TApplicative as WithTypeArg<T>>::Type> {
            let init = lift::<TApplicative, Vec<T>>(vec!());
            lfoldr(|x:Vec<T>, ys| llift2(|a, b| pure_append(a,b), x, ys), lift(vec!()), x)
        }}
fn pure_append<T>(elem: T, mut vec: Vec<T>) -> Vec<T> {
    vec.push(elem);
    vec
}
*/
