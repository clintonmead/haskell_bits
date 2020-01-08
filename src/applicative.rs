use crate::*;
use is_type::Is;

// This is haskell's "pure", but pure is a former keyword in Rust,
// and perhaps "lift" is easier for non Haskellers.
pub trait Lift {
    fn lift<T>(x: T) -> <Self as WithTypeArg<T>>::Type
    where
        Self: WithTypeArg<T>;
}

pub fn lift<TCon, T>(x: T) -> <TCon as WithTypeArg<T>>::Type
where
    TCon: Lift + WithTypeArg<T> + ?Sized,
    <TCon as WithTypeArg<T>>::Type: TypeApp<TCon, T>,
{
    <TCon as Lift>::lift::<T>(x)
}

pub fn lift_c<TCon, T, U>(x: U::Param) -> U
where
    TCon: Lift + WithTypeArg<T> + ?Sized,
    T: Is<Type = U::Param>,
    U: TypeApp<TCon, T>,
    <TCon as WithTypeArg<T>>::Type: TypeApp<TCon, T>,
{
    Is::from_val(lift::<TCon, T>(Is::from_val(x)))
}

pub trait SingletonApplicative: Lift {
    fn lap<TIn, TOut, TFunc>(
        f: <Self as WithTypeArg<TFunc>>::Type,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFunc: FnOnce(TIn) -> TOut,
    {
        <Self as SingletonApplicative>::llift2(|y1: TFunc, y2: TIn| y1(y2), f, x)
    }

    fn llift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: <Self as WithTypeArg<TIn1>>::Type,
        x2: <Self as WithTypeArg<TIn2>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn1> + WithTypeArg<TIn2> + WithTypeArg<TOut>,
        TFunc: FnOnce(TIn1, TIn2) -> TOut;
}

pub fn lap<TCon, TIn, TOut, TFunc, F, X>(
    f: F,
    x: X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: SingletonApplicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    TFunc: Fn(TIn) -> TOut,
    F: TypeApp<TCon, TFunc>,
    X: TypeApp<TCon, TIn>,
{
    <TCon as SingletonApplicative>::lap::<TIn, TOut, TFunc>(f.into_val(), x.into_val())
}

pub fn llift2<TCon, TIn1, TIn2, TOut, F, X1, X2>(
    f: F,
    x1: X1,
    x2: X2,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: SingletonApplicative
        + WithTypeArg<TIn1>
        + WithTypeArg<TIn2>
        + WithTypeArg<TOut>
        + WithTypeArg<F>
        + ?Sized,
    F: Fn(TIn1, TIn2) -> TOut,
    X1: TypeApp<TCon, TIn1>,
    X2: TypeApp<TCon, TIn2>
{
    <TCon as SingletonApplicative>::llift2(f, x1.into_val(), x2.into_val())
}

pub trait Applicative: Functor + Lift {
    fn ap<TIn, TOut, TFunc>(
        f: &<Self as WithTypeArg<TFunc>>::Type,
        x: &<Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFunc: Fn(&TIn) -> TOut,
    {
        <Self as Applicative>::lift2(|y1: &TFunc, y2: &TIn| y1(y2), f, x)
    }

    fn lift2<TIn1, TIn2, TOut, F>(
        f: F,
        x1: &<Self as WithTypeArg<TIn1>>::Type,
        x2: &<Self as WithTypeArg<TIn2>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn1> + WithTypeArg<TIn2> + WithTypeArg<TOut>,
        F: Fn(&TIn1, &TIn2) -> TOut;
}

// fap(f, x)
pub fn ap<TCon, TFunc, TIn, TOut, F, X>(
    f: &F,
    x: &X,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Applicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
    TFunc: Fn(&TIn) -> TOut,
    F: TypeApp<TCon, TFunc> + ?Sized,
    X: TypeApp<TCon, TIn> + ?Sized,
{
    <TCon as Applicative>::ap::<TIn, TOut, TFunc>(f.into_ref(), x.into_ref())
}

pub fn lift2<TCon, TIn1, TIn2, TOut, F, X1, X2>(
    f: F,
    x1: &X1,
    x2: &X2,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Applicative + WithTypeArg<TIn1> + WithTypeArg<TIn2> + WithTypeArg<TOut> + ?Sized,
    F: Fn(&TIn1, &TIn2) -> TOut,
    X1: TypeApp<TCon, TIn1> + ?Sized,
    X2: TypeApp<TCon, TIn2> + ?Sized,
{
    <TCon as Applicative>::lift2(f, x1.into_ref(), x2.into_ref())
}
