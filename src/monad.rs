use crate::*;
use is_type::Is;

// Monad
pub trait Monad: Applicative {
    fn bind<TIn, TOut, TFuncArg>(
        x: &<Self as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFuncArg: Fn(&TIn) -> <Self as WithTypeArg<TOut>>::Type;

    fn bind_ignore<TIn, TOut>(
        x: &<Self as WithTypeArg<TIn>>::Type,
        y: &<Self as WithTypeArg<TOut>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        <Self as WithTypeArg<TOut>>::Type: Clone,
    {
        <Self as Monad>::bind::<TIn, TOut, _>(x, |_| y.clone())
    }

    fn join<T>(
        x: &<Self as WithTypeArg<<Self as WithTypeArg<T>>::Type>>::Type,
    ) -> <Self as WithTypeArg<T>>::Type
    where
        Self: WithTypeArg<T> + WithTypeArg<<Self as WithTypeArg<T>>::Type>,
        <Self as WithTypeArg<T>>::Type: Clone,
    {
        <Self as Monad>::bind::<<Self as WithTypeArg<T>>::Type, T, _>(x, |y| y.clone())
    }
}

// LinearMonad
pub trait LinearMonad: LinearApplicative + Lift {
    fn lbind<TIn, TOut, TFuncArg>(
        x: <Self as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFuncArg: FnOnce(TIn) -> <Self as WithTypeArg<TOut>>::Type;

    fn lbind_ignore<TIn, TOut>(
        x: <Self as WithTypeArg<TIn>>::Type,
        y: <Self as WithTypeArg<TOut>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>,
    {
        <Self as LinearMonad>::lbind::<TIn, TOut, _>(x, |_| y)
    }

    fn ljoin<T>(
        x: <Self as WithTypeArg<<Self as WithTypeArg<T>>::Type>>::Type,
    ) -> <Self as WithTypeArg<T>>::Type
    where
        Self: WithTypeArg<T> + WithTypeArg<<Self as WithTypeArg<T>>::Type>,
    {
        <Self as LinearMonad>::lbind::<<Self as WithTypeArg<T>>::Type, T, _>(x, |y| y)
    }
}

// bind(x, f)
pub fn bind<TCon, TIn, TOut, TArg, TFuncArg, U>(x: &TArg, f: TFuncArg) -> U
where
    TArg: TypeApp<TCon, TIn>,
    TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut>,
    TFuncArg: Fn(&TIn) -> U,
    U: TypeApp<TCon, TOut>,
{
    Is::from_val(<TCon as Monad>::bind::<TIn, TOut, _>(x.into_ref(), |y| {
        f(y).into_val()
    }))
}

// bind_c(x, f)
pub fn bind_c<TCon, TIn, TOut, TFuncArg, U>(x: &<TCon as WithTypeArg<TIn>>::Type, f: TFuncArg) -> U
where
    TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut>,
    TFuncArg: Fn(&TIn) -> U,
    U: TypeApp<TCon, TOut>,
{
    bind(Is::into_ref(x), f)
}

// bind_ignore(x, y)
pub fn bind_ignore<TCon, TIn, TOut, U>(x: &impl TypeApp<TCon, TIn>, y: &U) -> U
where
    U: TypeApp<TCon, TOut>,
    TCon: Monad + WithTypeArg<TIn> + WithTypeArg<TOut>,
    <TCon as WithTypeArg<TOut>>::Type: Clone,
{
    Is::from_val(<TCon as Monad>::bind_ignore::<TIn, TOut>(
        x.into_ref(),
        y.into_ref(),
    ))
}

// join(x)

// "TInner: TypeApp<TCon, T>" implies "TInner = <TCon as WithTypeArg<TInner>>::Type",
// indeed this is what `Is::into()` and associated functions do.
//
// But here we've got:
//
// "TInner: TypeApp<TCon, T>"
//
// and we want to convert:
//
// X<TInner>
// to
// X<<TCon as WithTypeArg<TInner>>::Type>"
//
// Given the definition of `TypeApp` this should always be true (I believe it's impossible for
// it not to be true) but I can't work out how  to write this safely, without using `fmap`.
//
// But doing `fmap` on the structure will take time,
// whereas I want this to be a zero time operation.
//
// So we're naughty and use `transmute`. I believe this is still sound,
// tell me loudly if you find a case where it isn't.
fn into_functor_ref<TCon, T, TInner>(
    x: &<TCon as WithTypeArg<TInner>>::Type,
) -> &<TCon as WithTypeArg<<TCon as WithTypeArg<T>>::Type>>::Type
where
    TCon: Functor
        + WithTypeArg<T>
        + WithTypeArg<TInner>
        + WithTypeArg<<TCon as WithTypeArg<T>>::Type>,
    TInner: TypeApp<TCon, T>,
{
    unsafe { std::mem::transmute(x) }
}

pub fn join<TCon, T, TInner>(x: &impl TypeApp<TCon, TInner>) -> <TCon as WithTypeArg<T>>::Type
where
    TInner: TypeApp<TCon, T>,
    TCon:
        Monad + WithTypeArg<T> + WithTypeArg<TInner> + WithTypeArg<<TCon as WithTypeArg<T>>::Type>,
    <TCon as WithTypeArg<T>>::Type: Clone,
{
    <TCon as Monad>::join::<T>(into_functor_ref::<TCon, T, TInner>(x.into_ref()))
}

// lbind(x, f)
pub fn lbind<TCon, TIn, TOut, TFuncArg, TFuncOut>(
    x: impl TypeApp<TCon, TIn>,
    f: TFuncArg,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearMonad + WithTypeArg<TIn> + WithTypeArg<TOut>,
    TFuncArg: FnOnce(TIn) -> TFuncOut,
    TFuncOut: TypeApp<TCon, TOut>,
{
    <TCon as LinearMonad>::lbind::<TIn, TOut, _>(x.into_val(), |y| f(y).into_val())
}

// lbind_ignore(x, y)
pub fn lbind_ignore<TCon, TIn, TOut>(
    x: impl TypeApp<TCon, TIn>,
    y: impl TypeApp<TCon, TOut>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearMonad + WithTypeArg<TIn> + WithTypeArg<TOut>,
{
    <TCon as LinearMonad>::lbind_ignore::<TIn, TOut>(x.into_val(), y.into_val())
}
