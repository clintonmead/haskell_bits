pub trait Semigroup where {
    fn semigroup_op(self : Self, other : Self) -> Self;
}