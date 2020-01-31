use crate::*;

pub struct TypeCon<E> {
    _unused: std::marker::PhantomData<E>,
}

impl<T, E> TypeAppParam for Result<T, E> {
    type Param = T;
}

impl<T, E> TypeApp<TypeCon<E>, T> for Result<T, E> {}

impl<T, E> WithTypeArg<T> for TypeCon<E> {
    type Type = Result<T, E>;
}

impl<E> LinearFunctor for TypeCon<E> {
    fn lmap<TIn, TOut, F>(
        f: F,
        x: <TypeCon<E> as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        F: Fn(TIn) -> TOut,
    {
        Result::map(x, f)
    }
}

impl<E> Functor for TypeCon<E>
where
    E: Clone,
{
    fn fmap<TIn, TOut, F>(
        f: F,
        x: &<TypeCon<E> as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        F: Fn(&TIn) -> TOut,
    {
        Result::map(x.as_ref(), f).clone_err()
    }
}

impl<E> Lift for TypeCon<E> {
    fn lift<T>(x: T) -> <TypeCon<E> as WithTypeArg<T>>::Type {
        Ok(x)
    }
}

impl<E> LinearApplicative for TypeCon<E> {
    fn lap<TIn, TOut, TFunc>(
        f: <TypeCon<E> as WithTypeArg<TFunc>>::Type,
        x: <TypeCon<E> as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        TFunc: FnOnce(TIn) -> TOut,
    {
        f.and_then(|f_val| x.map(|x_val| f_val(x_val)))
    }

    fn llift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: <TypeCon<E> as WithTypeArg<TIn1>>::Type,
        x2: <TypeCon<E> as WithTypeArg<TIn2>>::Type,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        TFunc: FnOnce(TIn1, TIn2) -> TOut,
    {
        x1.and_then(|x1val| x2.map(|x2val| f(x1val, x2val)))
    }
}

impl<E> Applicative for TypeCon<E>
where
    E: Clone,
{
    fn ap<TIn, TOut, TFunc>(
        f: &<TypeCon<E> as WithTypeArg<TFunc>>::Type,
        x: &<TypeCon<E> as WithTypeArg<TIn>>::Type,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn) -> TOut,
    {
        f.as_ref()
            .clone_err()
            .and_then(|f_val| fmap(f_val, x))
    }

    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<TypeCon<E> as WithTypeArg<TIn1>>::Type,
        x2: &<TypeCon<E> as WithTypeArg<TIn2>>::Type,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn1, &TIn2) -> TOut,
    {
        x1.as_ref()
            .and_then(|x1_val| x2.as_ref().map(|x2_val| f(x1_val, x2_val)))
            .clone_err()
    }
}

impl<E> LinearMonad for TypeCon<E> {
    fn lbind<TIn, TOut, F>(
        x: <TypeCon<E> as WithTypeArg<TIn>>::Type,
        f: F,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        F: FnOnce(TIn) -> <TypeCon<E> as WithTypeArg<TOut>>::Type,
    {
        x.and_then(f)
    }
}

impl<E> Monad for TypeCon<E>
where
    E: Clone,
{
    fn bind<TIn, TOut, F>(
        x: &<TypeCon<E> as WithTypeArg<TIn>>::Type,
        f: F,
    ) -> <TypeCon<E> as WithTypeArg<TOut>>::Type
    where
        F: Fn(&TIn) -> <TypeCon<E> as WithTypeArg<TOut>>::Type,
    {
        x.as_ref().clone_err().and_then(f)
    }
}

trait CloneError<T, E>
where
    E: Clone,
{
    fn clone_err(self) -> Result<T, E>;
}

impl<T, E> CloneError<T, E> for Result<T, &E>
where
    E: Clone,
{
    fn clone_err(self) -> Result<T, E> {
        self.map_err(|err| Clone::clone(err))
    }
}
