use crate::*;
use is_type::Is;

pub struct Ref {}
pub struct Val {}

pub trait TypeAppParam {
    type Param;
}

pub trait TypeApp<TCon, T>
where
    TCon: WithTypeArg<T> + ?Sized,
    Self: Is<Type = <TCon as WithTypeArg<T>>::Type> + TypeAppParam,
    T: ?Sized,
{
}

// This is useful for traits like CallMap where you're not sure whether your impls are either taking
// reference or value arguments.
// There might be a cleaner approach to this but I haven't found it.
pub trait TypeAppMaybeRef<TCon, T, RefT>
where
    TCon: WithTypeArg<T> + ?Sized,
    T: ?Sized,
    RefT: ?Sized
{
}

impl<TCon, T, TCollection> TypeAppMaybeRef<TCon, T, Val> for TCollection
where
    TCollection: TypeApp<TCon, T>,
    TCon: WithTypeArg<T> + ?Sized,
    T: ?Sized
{
}

impl<TCon, T, TCollection> TypeAppMaybeRef<TCon, T, Ref> for &TCollection
where
    TCollection: TypeApp<TCon, T>,
    TCon: WithTypeArg<T> + ?Sized,
    T: ?Sized
{
}
