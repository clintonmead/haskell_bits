use crate::*;

pub trait LinearTraversable: LinearFunctor + LinearFoldable {
    fn sequence<TApplicative, T>(
        x: <Self as WithTypeArg<<TApplicative as WithTypeArg<T>>::Type>>::Type,
    ) -> <TApplicative as WithTypeArg<<Self as WithTypeArg<T>>::Type>>::Type
    where
        TApplicative:
            LinearApplicative + WithTypeArg<T> + WithTypeArg<<Self as WithTypeArg<T>>::Type>,
        Self: WithTypeArg<T> + WithTypeArg<<TApplicative as WithTypeArg<T>>::Type>;

    fn traverse<TApplicative, TIn, TOut, F>(
        f: F,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <TApplicative as WithTypeArg<<Self as WithTypeArg<TOut>>::Type>>::Type
    where
        F: Fn(TIn) -> <TApplicative as WithTypeArg<TOut>>::Type,
        TApplicative:
            LinearApplicative + WithTypeArg<<Self as WithTypeArg<TOut>>::Type> + WithTypeArg<TOut>,
        Self: WithTypeArg<TIn>
            + WithTypeArg<TOut>
            + WithTypeArg<<TApplicative as typeapp::WithTypeArg<TOut>>::Type>,
    {
        sequence(lmap(f, x))
    }
}

pub fn traverse<TCon, TApplicative, TIn, TOut, F, X>(
    f: F,
    x: X,
) -> <TApplicative as WithTypeArg<<TCon as WithTypeArg<TOut>>::Type>>::Type
where
    F: Fn(TIn) -> <TApplicative as WithTypeArg<TOut>>::Type,
    X: TypeApp<TCon, TIn>,
    TApplicative:
        LinearApplicative + WithTypeArg<TOut> + WithTypeArg<<TCon as WithTypeArg<TOut>>::Type>,
    TCon: LinearTraversable
        + WithTypeArg<TIn>
        + WithTypeArg<TOut>
        + WithTypeArg<<TApplicative as typeapp::WithTypeArg<TOut>>::Type>
        + ?Sized,
{
    <TCon as LinearTraversable>::traverse::<TApplicative, TIn, TOut, F>(f, x.into_val())
}

pub fn sequence<TCon, TApplicative, T, X, Y>(
    x: X,
) -> <TApplicative as WithTypeArg<<TCon as WithTypeArg<T>>::Type>>::Type
where
    TApplicative: LinearApplicative + WithTypeArg<T> + WithTypeArg<<TCon as WithTypeArg<T>>::Type>,
    TCon: LinearTraversable
        + WithTypeArg<T>
        + WithTypeArg<<TApplicative as WithTypeArg<T>>::Type>
        + WithTypeArg<Y>
        + ?Sized,
    X: TypeApp<TCon, Y>,
    Y: TypeApp<TApplicative, T>,
{
    <TCon as LinearTraversable>::sequence::<TApplicative, T>(lmap(|y| y.into_val(), x.into_val()))
}
