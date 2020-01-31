pub mod applicative;
pub mod ext;
pub mod functor;
pub mod impls;
pub mod monad;
pub mod typeapp;

pub use applicative::*;
pub use ext::*;
pub use functor::*;
pub use impls::*;
pub use monad::*;
pub use typeapp::*;

#[doc(hidden)]
pub mod mdo;
#[doc(hidden)]
pub use mdo::*;

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test() {
        // Shows both Vector and Option Functors are working
        let v: Vec<Option<u64>> = vec![Some(42), None];
        let v3 = v.clone();
        let v2: Vec<Option<u64>> = map(|x| map(|y| y * 2, x), v);
        let _v4: Vec<Option<u64>> = v3.mapop(|x| map(|y| y * 2, x));
        assert_eq!(v2, vec!(Some(84), None));

        let f: fn(&u32) -> u32 = |x| x * 2;
        let g: fn(&u32) -> u32 = |x| x + 3;
        let h: fn(&u32) -> u32 = |x| x * 2;

        let test_vec: Vec<u32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let test_option: Option<u32> = Some(42);
        let _result_vec = map2(h, g, &test_vec);
        let _result_option = map2(h, g, &test_option);

        let x1: Vec<u32> = vec![1, 2, 3];
        let x2: Vec<u32> = vec![1, 2, 3];
        let x3: Vec<u32> = vec![1, 2, 3];
        let x4: Vec<u32> = vec![1, 2, 3];

        let _result1: Vec<u32> = map(f, &x1);
        let _result2: Vec<u32> = map(f, x2);
        let _result3: Vec<u32> = map(h, &x3);
        let _result4: Vec<u32> = map(h, x4);

        let _o_lift: Option<u32> = lift_c(4);

        let o1: Option<u32> = lift_c(5);
        let o2: Option<u32> = lift_c(7);

        let _o3 = lbind(o1, |x| fmap(|y| x + y, &o1));

        let _o4: Option<u32> = bind_c(&lift_c(5), |x: &_| lift_c(Clone::clone(x)));
        let _o5: Option<u32> = (|x: _| Some(Clone::clone(x)))(&5);

        let _do_result: Option<(u32, u32)> = mdo_c! {
            x =<< &o1;
            y =<< &o2;
            ret (Clone::clone(x), Clone::clone(y));
        };

        let v1: Vec<u32> = vec![1, 2, 3];
        let v2: Vec<u32> = vec![4, 5, 6];

        let o1: Option<u32> = Some(1);
        let o2: Option<u32> = Some(2);
        let o3: Option<u32> = None;

        let of: Option<_> = Some(|x: &u32| x.clone());

        let v_result = monadic_pair(&v1, &v2);
        let o1_result = monadic_pair(&o1, &o2);
        let o2_result = monadic_pair(&o1, &o3);

        let _applicative_result = (|x: &_| {
            let z = Clone::clone(x);
            move |y: &_| z * y
        })
        .fmap(&o1)
        .ap(&o2);
        let _applicative_option_result = (|x| move |y| x + y).lmap(o1).lap(o2);
        assert_eq!(Some(3), _applicative_option_result);

        let _ = ap(&of, &o1);

        assert_eq!(
            v_result,
            vec![
                (1, 4),
                (1, 5),
                (1, 6),
                (2, 4),
                (2, 5),
                (2, 6),
                (3, 4),
                (3, 5),
                (3, 6)
            ]
        );
        assert_eq!(o1_result, Some((1, 2)));
        assert_eq!(o2_result, None);
    }

    fn map2<TIn, TMid, TOut, TCon>(
        f: impl Fn(&TIn) -> TMid,
        g: impl Fn(&TMid) -> TOut,
        x: &impl TypeApp<TCon, TIn>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: Functor + WithTypeArg<TIn> + WithTypeArg<TMid> + WithTypeArg<TOut>,
        TIn: Clone,
    {
        map(g, map(f, x))
    }

    fn _lmap2<TIn, TMid, TOut, TCon>(
        f: impl Fn(TIn) -> TMid,
        g: impl Fn(TMid) -> TOut,
        x: impl TypeApp<TCon, TIn>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TMid> + WithTypeArg<TOut>,
    {
        lmap(g, lmap(f, x))
    }

    fn monadic_pair<TCon, T, TArg>(x: &TArg, y: &TArg) -> <TCon as WithTypeArg<(T, T)>>::Type
    where
        TCon: Monad + WithTypeArg<T> + WithTypeArg<(T, T)>,
        TArg: TypeApp<TCon, T>,
        T: Clone,
    {
        mdo! {
            x_val =<< x;
            y_val =<< y;
            ret<TCon> (Clone::clone(x_val), Clone::clone(y_val));
        }
    }
}
