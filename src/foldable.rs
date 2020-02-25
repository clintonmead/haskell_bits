use crate::*;

pub trait LinearFoldable {
    fn lfoldr<F, TIn, TOut>(f: F, init: TOut, x: <Self as WithTypeArg<TIn>>::Type) -> TOut
    where
        F: Fn(TIn, TOut) -> TOut,
        Self: WithTypeArg<TIn>;
}

pub fn lfoldr<TCon, F, TIn, TOut, X>(f: F, init: TOut, x: X) -> TOut
where
    F: Fn(TIn, TOut) -> TOut,
    TCon: LinearFoldable + WithTypeArg<TIn>,
    X: TypeApp<TCon, TIn>,
{
    <TCon as LinearFoldable>::lfoldr(f, init, x.into_val())
}

pub trait Foldable {
    fn foldr<F, TIn, TOut>(f: F, init: TOut, x: &<Self as WithTypeArg<TIn>>::Type) -> TOut
    where
        F: Fn(&TIn, TOut) -> TOut,
        Self: WithTypeArg<TIn>;
}

pub fn foldr<TCon, F, TIn, TOut, X>(f: F, init: TOut, x: &X) -> TOut
where
    F: Fn(&TIn, TOut) -> TOut,
    TCon: Foldable + WithTypeArg<TIn>,
    X: TypeApp<TCon, TIn> + ?Sized,
{
    <TCon as Foldable>::foldr(f, init, x.into_ref())
}
