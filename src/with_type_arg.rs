use crate::*;

pub trait WithTypeArg<T : ?Sized>
{
    type Type: TypeApp<Self, T>;
}
