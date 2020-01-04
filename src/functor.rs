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

// This is for f.lmap(x) syntax
pub trait LMapExt {
    fn lmap<TCon, TIn, TOut>(
        self,
        x: impl TypeApp<TCon, TIn> + Sized,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: Fn(TIn) -> TOut + Sized,
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut>,
    {
        lmap(self, x)
    }
}

impl<T> LMapExt for T {}

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

// This is for x.lmapop(f) syntax
pub trait LMapOpExt {
    fn lmapop<TCon, TIn, TOut>(self, f: impl Fn(TIn) -> TOut) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TIn> + Sized,
        TCon: LinearFunctor + WithTypeArg<TIn>,
        TCon: WithTypeArg<TOut>,
    {
        lmap(f, self)
    }
}

impl<T> LMapOpExt for T {}

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

// This is for f.fmap(x) syntax
pub trait FMapExt {
    fn fmap<TCon, TIn, TOut>(self, x: &impl TypeApp<TCon, TIn>) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: Fn(&TIn) -> TOut + Sized,
        TCon: Functor + WithTypeArg<TIn>,
        TCon: WithTypeArg<TOut>,
    {
        fmap(self, x)
    }
}

impl<T> FMapExt for T {}

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

// This is for x.fmapop(f) syntax
pub trait FMapOpExt {
    fn fmapop<TCon, TIn, TOut>(&self, f: impl Fn(&TIn) -> TOut) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TIn> + Sized,
        TCon: Functor + WithTypeArg<TIn>,
        TCon: WithTypeArg<TOut>,
    {
        fmap(f, self)
    }
}

impl<T> FMapOpExt for T {}

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
