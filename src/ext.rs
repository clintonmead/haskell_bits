use crate::*;

pub trait UnsizedExt {
    fn ap<TCon, TIn, TOut, TFunc, X>(
        &self,
        x: &X
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TFunc>,
        TCon: Applicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFunc: Fn(&TIn) -> TOut,
        X: TypeApp<TCon, TIn> + ?Sized,
    {
        ap(self, x)
    }

    fn bind<TCon, TIn, TOut, F, TResult>(&self, f: F) -> TResult
    where
        TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        F: Fn(&TIn) -> TResult,
        TResult: TypeApp<TCon, TOut>,
        Self: TypeApp<TCon, TIn>,
    {
        bind(self, f)
    }

    fn bind_ignore<TCon, TIn, TOut, TResult>(&self, y: &TResult) -> TResult
    where
        TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        Self: TypeApp<TCon, TIn>,
        TResult: TypeApp<TCon, TOut>,
        <TCon as WithTypeArg<TOut>>::Type: Clone,
    {
        bind_ignore(self, y)
    }

    fn fjoin<TCon, T, TInner>(&self) -> <TCon as WithTypeArg<T>>::Type
    where
        TCon: Monad
            + WithTypeArg<T>
            + WithTypeArg<TInner>
            + WithTypeArg<<TCon as WithTypeArg<T>>::Type>
            + ?Sized,
        Self: TypeApp<TCon, TInner>,
        TInner: TypeApp<TCon, T>,
        <TCon as WithTypeArg<T>>::Type: Clone,
    {
        fjoin(self)
    }
}

impl<T> UnsizedExt for T {}

pub trait SizedExt: Sized {
    fn lift<TCon>(self) -> <TCon as WithTypeArg<Self>>::Type
    where
        TCon: Lift + WithTypeArg<Self> + ?Sized,
    {
        lift::<TCon, Self>(self)
    }

    fn lap<TCon, TIn, TOut, TFunc, X>(
        self,
        x: X,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: LinearApplicative
            + WithTypeArg<TFunc>
            + WithTypeArg<TIn>
            + WithTypeArg<TOut>
            + ?Sized,
        Self: TypeApp<TCon, TFunc>,
        TFunc: Fn(TIn) -> TOut,
        X: TypeApp<TCon, TIn>,
    {
        <TCon as LinearApplicative>::lap::<TIn, TOut, TFunc>(self.into_val(), x.into_val())
    }

    fn lmap<TCon, TIn, TOut, X>(
        self,
        x: X,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        Self: Fn(TIn) -> TOut,
        X: TypeApp<TCon, TIn> 
    {
        lmap(self, x)
    }

    fn lmapop<TCon, TIn, TOut, F>(self, f: F) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        Self: TypeApp<TCon, TIn>,
        F: Fn(TIn) -> TOut
    {
        lmap(f, self)
    }

    fn fmap<TCon, TIn, TOut, X>(self, x: &X) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        Self: Fn(&TIn) -> TOut,
        X: TypeApp<TCon, TIn> + ?Sized
    {
        fmap(self, x)
    }

    fn fmapop<TCon, TIn, TOut, F>(&self, f: F) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        Self: TypeApp<TCon, TIn>,
        F: Fn(&TIn) -> TOut
    {
        fmap(f, self)
    }

    fn lbind<TCon, TIn, TOut, F, TResult>(self, f: F) -> TResult
    where
        TCon: LinearMonad + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        F: Fn(TIn) -> TResult,
        TResult: TypeApp<TCon, TOut>,
        Self: TypeApp<TCon, TIn>,
    {
        lbind(self, f)
    }

    fn lbind_ignore<TCon, TIn, TOut, TResult>(self, y: &TResult) -> TResult
    where
        TCon: LinearMonad + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
        Self: TypeApp<TCon, TIn>,
        TResult: TypeApp<TCon, TOut>,
        <TCon as WithTypeArg<TOut>>::Type: Clone,
    {
        lbind_ignore(self, y)
    }
}

impl<T: Sized> SizedExt for T {}
