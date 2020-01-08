use crate::*;

// Implement this trait for LinearFunctor
pub trait LinearFunctor {
    fn lmap<TIn, TOut, F>(
        f: F,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        F: Fn(TIn) -> TOut;

    fn lmapconst<TIn, TOut>(
        e: &TOut,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        TOut: Clone,
    {
        <Self as LinearFunctor>::lmap(|_: TIn| e.clone(), x)
    }
}

// Implement this trait for Functor
pub trait Functor: LinearFunctor {
    fn fmap<TIn, TOut, F>(
        f: F,
        x: &<Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        F: Fn(&TIn) -> TOut;

    fn fmapconst<TIn, TOut>(
        e: &TOut,
        x: &<Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        TOut: Clone,
    {
        Self::fmap(|_: &TIn| e.clone(), x)
    }
}

// Call this for lmap(f, x) syntax
pub fn lmap<TCon, TIn, TOut, F, X>(
    f: F,
    x: X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    F: Fn(TIn) -> TOut,
    X: TypeApp<TCon, TIn>,
{
    <TCon as LinearFunctor>::lmap(f, x.into_val())
}

// And for lmapconst(e, x)
pub fn lmapconst<TCon, TIn, TOut, X>(
    e: &TOut,
    x: X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    TOut: Clone,
    X: TypeApp<TCon, TIn>,
{
    <TCon as LinearFunctor>::lmapconst::<TIn, TOut>(e, x.into_val())
}

// Call this for fmap(f, x) syntax
pub fn fmap<TCon, TIn, TOut, F, X>(
    f: F,
    x: &X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    X: TypeApp<TCon, TIn> + ?Sized,
    F: Fn(&TIn) -> TOut,
{
    TCon::fmap(f, x.into_ref())
}

// And for fmapconst(e, x)
pub fn fmapconst<TCon, TIn, TOut, X>(
    e: &TOut,
    x: &X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    TOut: Clone + ?Sized,
    X: TypeApp<TCon, TIn>
{
    TCon::fmapconst::<TIn, TOut>(e, x.into_ref())
}

// This allows you to make a x.map(f) call which will work which will call either
// fmap or lmap depending on the arguments
pub trait MapExt<TCon, F, TIn, TOut, TFuncIn, TIsRef>
where
    TCon: WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    Self: TypeAppMaybeRef<TCon, TIn, TIsRef>,
    F: Fn(TFuncIn) -> TOut,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type;
}

impl<TCon, TIn, TOut, F, X> MapExt<TCon, F, TIn, TOut, TIn, Val> for X
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    X: TypeApp<TCon, TIn>,
    F: Fn(TIn) -> TOut,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        lmap(f, self)
    }
}

impl<TCon, TIn, TOut, F, X> MapExt<TCon, F, TIn, TOut, &TIn, Val> for X
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    X: TypeApp<TCon, TIn>,
    F: Fn(&TIn) -> TOut,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        lmap(|y| f(&y), self)
    }
}

impl<TCon, TIn, TOut, F, X> MapExt<TCon, F, TIn, TOut, TIn, Ref> for &X
where
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut>,
    X: TypeApp<TCon, TIn>,
    F: Fn(TIn) -> TOut,
    TIn: Clone,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        fmap(|y| f(y.clone()), self)
    }
}

impl<TCon, TIn, TOut, F, X> MapExt<TCon, F, TIn, TOut, &TIn, Ref> for &X
where
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    X: TypeApp<TCon, TIn>,
    F: Fn(&TIn) -> TOut,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        fmap(f, self)
    }
}

// map(f, x)
pub fn map<TCon, F, TIn, TOut, TFuncIn, TIsRef, X>(
    f: F,
    x: X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    F: Fn(TFuncIn) -> TOut,
    X: MapExt<TCon, F, TIn, TOut, TFuncIn, TIsRef>,
{
    X::mapop(x, f)
}
