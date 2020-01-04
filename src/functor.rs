use crate::*;

// Implement this trait for LinearFunctor
pub trait LinearFunctor {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>;

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
    fn fmap<TIn, TOut>(
        f: impl Fn(&TIn) -> TOut,
        x: &<Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>;

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
pub fn lmap<TCon, TIn, TOut>(
    f: impl Fn(TIn) -> TOut,
    x: impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
{
    <TCon as LinearFunctor>::lmap(f, x.into_val())
}

// And for lmapconst(e, x)
pub fn lmapconst<TCon, TIn, TOut>(
    e: &TOut,
    x: impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut>,
    TOut: Clone,
{
    <TCon as LinearFunctor>::lmapconst::<TIn, TOut>(e, x.into_val())
}

// Call this for fmap(f, x) syntax
pub fn fmap<TCon, TIn, TOut, TFunc, TParam>(
    f: TFunc,
    x: &TParam,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    TParam: TypeApp<TCon, TIn>,
    TFunc: Fn(&TIn) -> TOut,
{
    TCon::fmap(f, x.into_ref())
}

// And for fmapconst(e, x)
pub fn fmapconst<TCon, TIn, TOut>(
    e: &TOut,
    x: impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    TOut: Clone,
{
    TCon::fmapconst::<TIn, TOut>(e, x.into_ref())
}

// This allows you to make a x.map(f) call which will work which will call either
// fmap or lmap depending on the arguments
pub trait MapExt<TCon, F, TIn, TOut, TFuncArgIn, TIsRef>
where
    Self: TypeAppMaybeRef<TCon, TIn, TIsRef>,
    F: Fn(TFuncArgIn) -> TOut,
    TCon: WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type;
}

impl<TCon, F, X, TIn, TOut> MapExt<TCon, F, TIn, TOut, TIn, Val> for X
where
    X: TypeApp<TCon, TIn>,
    F: Fn(TIn) -> TOut,
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        lmap(f, self)
    }
}

impl<TCon, F, X, TIn, TOut> MapExt<TCon, F, TIn, TOut, &TIn, Val> for X
where
    X: TypeApp<TCon, TIn>,
    F: Fn(&TIn) -> TOut,
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        lmap(|y| f(&y), self)
    }
}

impl<TCon, F, X, TIn, TOut> MapExt<TCon, F, TIn, TOut, &TIn, Ref> for &X
where
    X: TypeApp<TCon, TIn>,
    F: Fn(&TIn) -> TOut,
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        fmap(f, self)
    }
}

impl<TCon, F, X, TIn, TOut> MapExt<TCon, F, TIn, TOut, TIn, Ref> for &X
where
    X: TypeApp<TCon, TIn>,
    F: Fn(TIn) -> TOut,
    TIn: Clone,
    TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut>,
{
    fn mapop(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type {
        fmap(|y| f(y.clone()), self)
    }
}

// map(f, x)
pub fn map<TCon, F, TIn, TOut, TFuncArgIn, TCollectionIsRef, X>(
    f: F,
    x: X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    F: Fn(TFuncArgIn) -> TOut,
    X: MapExt<TCon, F, TIn, TOut, TFuncArgIn, TCollectionIsRef>,
    TCon: WithTypeArg<TIn> + WithTypeArg<TOut>,
{
    X::mapop(x, f)
}
