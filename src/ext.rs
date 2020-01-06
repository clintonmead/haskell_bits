use crate::*;

pub trait FApExt {
    fn ap<TCon, TIn, TOut, TFunc>(
        &self,
        x: &impl TypeApp<TCon, TIn>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TFunc>,
        TCon: Applicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFunc: Fn(&TIn) -> TOut,
    {
        <TCon as Applicative>::ap::<TIn, TOut, TFunc>(self.into_ref(), x.into_ref())
    }
}

impl<T> FApExt for T {}

pub trait SizedExt: Sized {
    fn lift<TCon>(self) -> <TCon as WithTypeArg<Self>>::Type
    where
        TCon: Lift + WithTypeArg<Self>,
    {
        lift::<TCon, Self>(self)
    }

    fn lap<TCon, TIn, TOut, TFunc>(
        self,
        x: impl TypeApp<TCon, TIn>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TFunc>,
        TCon: LinearApplicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFunc: Fn(TIn) -> TOut,
    {
        <TCon as LinearApplicative>::lap::<TIn, TOut, TFunc>(self.into_val(), x.into_val())
    }

    fn lmap<TCon, TIn, TOut>(
        self,
        x: impl TypeApp<TCon, TIn> + Sized,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: Fn(TIn) -> TOut,
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut>,
    {
        lmap(self, x)
    }

    fn lmapop<TCon, TIn, TOut>(self, f: impl Fn(TIn) -> TOut) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TIn>,
        TCon: LinearFunctor + WithTypeArg<TIn>,
        TCon: WithTypeArg<TOut>,
    {
        lmap(f, self)
    }

    fn fmap<TCon, TIn, TOut>(self, x: &impl TypeApp<TCon, TIn>) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: Fn(&TIn) -> TOut,
        TCon: Functor + WithTypeArg<TIn>,
        TCon: WithTypeArg<TOut>,
    {
        fmap(self, x)
    }

    fn fmapop<TCon, TIn, TOut>(&self, f: impl Fn(&TIn) -> TOut) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TIn>,
        TCon: Functor + WithTypeArg<TIn>,
        TCon: WithTypeArg<TOut>,
    {
        fmap(f, self)
    }

    fn bind<TCon, TIn, TOut, TFuncArg, U>(&self, f: TFuncArg) -> U
    where
        TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFuncArg: Fn(&TIn) -> U,
        U: TypeApp<TCon, TOut>,
        Self: TypeApp<TCon, TIn>,
    {
        bind(self, f)
    }

    fn bind_ignore<TCon, TIn, TOut, U>(&self, y: &U) -> U
    where
        U: TypeApp<TCon, TOut>,
        Self: TypeApp<TCon, TIn>,
        TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut>,
        <TCon as WithTypeArg<TOut>>::Type: Clone,
    {
        bind_ignore(self, y)
    }

    fn join<TCon, T, TInner>(&self) -> <TCon as WithTypeArg<T>>::Type
    where
        Self: TypeApp<TCon, TInner>,
        TInner: TypeApp<TCon, T>,
        TCon: Monad
            + WithTypeArg<T>
            + WithTypeArg<TInner>
            + WithTypeArg<<TCon as WithTypeArg<T>>::Type>,
        <TCon as WithTypeArg<T>>::Type: Clone,
    {
        join(self)
    }

    fn lbind<TCon, TIn, TOut, TFuncArg, TFuncOut>(
        self,
        f: TFuncArg,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: LinearMonad + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFuncArg: FnOnce(TIn) -> TFuncOut,
        TFuncOut: TypeApp<TCon, TOut>,
        Self: TypeApp<TCon, TIn>,
    {
        lbind(self, f)
    }

    fn lbind_ignore<TCon, TIn, TOut>(
        self,
        y: impl TypeApp<TCon, TOut>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: TypeApp<TCon, TIn>,
        TCon: LinearMonad + WithTypeArg<TIn> + WithTypeArg<TOut>,
    {
        lbind_ignore(self, y)
    }
}

impl<T: Sized> SizedExt for T {}
