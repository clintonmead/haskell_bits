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
    <TCon as WithTypeArg<T>>::Type: TypeApp<TCon, T>,
    TCon: Lift + WithTypeArg<T>,
{
    <TCon as Lift>::lift::<T>(x)
}

pub fn lift_c<TCon, T, U>(x: U::Param) -> U
where
    T: Is<Type = U::Param>,
    U: TypeApp<TCon, T>,
    <TCon as WithTypeArg<T>>::Type: TypeApp<TCon, T>,
    TCon: Lift + WithTypeArg<T>,
{
    Is::from_val(lift::<TCon, T>(Is::from_val(x)))
}

pub trait ApplicativeExt {
    // x.lift()
    fn lift<TCon>(self) -> <TCon as WithTypeArg<Self>>::Type
    where
        TCon: Lift + WithTypeArg<Self>,
        Self: Sized,
    {
        lift::<TCon, Self>(self)
    }

    // f.lap(x)
    fn lap<TCon, TIn, TOut, TFunc>(
        self,
        x: impl TypeApp<TCon, TIn>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: Sized + TypeApp<TCon, TFunc>,
        TCon: LinearApplicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
        TFunc: Fn(TIn) -> TOut,
    {
        <TCon as LinearApplicative>::lap::<TIn, TOut, TFunc>(self.into_val(), x.into_val())
    }

    // f.ap(x)
    fn ap<TCon, TIn, TOut, TFunc>(
        self: &Self,
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

impl<T> ApplicativeExt for T {}

pub trait LinearApplicative: Lift {
    fn lap<TIn, TOut, TFunc>(
        f: <Self as WithTypeArg<TFunc>>::Type,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut> + Sized,
        TFunc: FnOnce(TIn) -> TOut,
    {
        <Self as LinearApplicative>::llift2(|y1: TFunc, y2: TIn| y1(y2), f, x)
    }

    fn llift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: <Self as WithTypeArg<TIn1>>::Type,
        x2: <Self as WithTypeArg<TIn2>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn1> + WithTypeArg<TIn2> + WithTypeArg<TOut> + Sized,
        TFunc: FnOnce(TIn1, TIn2) -> TOut;
}

pub fn lap<TCon, TIn, TOut, TFunc>(
    f: impl TypeApp<TCon, TFunc>,
    x: impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearApplicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
    TFunc: Fn(TIn) -> TOut,
{
    <TCon as LinearApplicative>::lap::<TIn, TOut, TFunc>(f.into_val(), x.into_val())
}

pub fn llift2<TCon, TIn1, TIn2, TOut, TFunc>(
    f: TFunc,
    x1: <TCon as WithTypeArg<TIn1>>::Type,
    x2: <TCon as WithTypeArg<TIn2>>::Type,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearApplicative
        + WithTypeArg<TIn1>
        + WithTypeArg<TIn2>
        + WithTypeArg<TOut>
        + WithTypeArg<TFunc>
        + Sized,
    TFunc: Fn(TIn1, TIn2) -> TOut,
{
    <TCon as LinearApplicative>::llift2(f, x1, x2)
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

    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<Self as WithTypeArg<TIn1>>::Type,
        x2: &<Self as WithTypeArg<TIn2>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn1> + WithTypeArg<TIn2> + WithTypeArg<TOut>,
        TFunc: Fn(&TIn1, &TIn2) -> TOut;
}

// fap(f, x)
pub fn ap<TCon, TIn, TOut, TFunc>(
    f: &impl TypeApp<TCon, TFunc>,
    x: &impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Applicative + WithTypeArg<TFunc> + WithTypeArg<TIn> + WithTypeArg<TOut>,
    TFunc: Fn(&TIn) -> TOut,
{
    <TCon as Applicative>::ap::<TIn, TOut, TFunc>(f.into_ref(), x.into_ref())
}

pub fn lift2<TCon, TIn1, TIn2, TOut, TFunc>(
    f: TFunc,
    x1: &impl TypeApp<TCon, TIn1>,
    x2: &impl TypeApp<TCon, TIn2>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: Applicative + WithTypeArg<TIn1> + WithTypeArg<TIn2> + WithTypeArg<TOut>,
    TFunc: Fn(&TIn1, &TIn2) -> TOut,
{
    <TCon as Applicative>::lift2(f, x1.into_ref(), x2.into_ref())
}
