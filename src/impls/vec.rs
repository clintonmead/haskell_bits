use crate::*;

pub struct VecTypeCon;

impl<T> TypeAppParam for Vec<T> {
    type Param = T;
}

impl<T> TypeApp<VecTypeCon, T> for Vec<T> {}

impl<T> WithTypeArg<T> for VecTypeCon {
    type Type = Vec<T>;
}

impl Functor for VecTypeCon {
    fn fmap<TIn, TOut>(
        f: impl Fn(&TIn) -> TOut,
        x: &<VecTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <VecTypeCon as WithTypeArg<TOut>>::Type {
        let size = x.capacity();
        let mut v: Vec<TOut> = Vec::with_capacity(size);
        for e in x {
            v.push(f(e));
        }
        v
    }
}

// This looks the same as the non-linear version but the for-loop here
// actually the for loop here is taking elements out of the original vector
// by value
impl LinearFunctor for VecTypeCon {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <VecTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <VecTypeCon as WithTypeArg<TOut>>::Type {
        let size = x.capacity();
        let mut v: Vec<TOut> = Vec::with_capacity(size);
        for e in x {
            v.push(f(e));
        }
        v
    }
}

impl Lift for VecTypeCon {
    fn lift<T>(x: T) -> <VecTypeCon as WithTypeArg<T>>::Type {
        vec![x]
    }
}

impl Applicative for VecTypeCon {
    fn lift2<TIn1, TIn2, TOut, TFunc>(
        f: TFunc,
        x1: &<VecTypeCon as WithTypeArg<TIn1>>::Type,
        x2: &<VecTypeCon as WithTypeArg<TIn2>>::Type,
    ) -> <VecTypeCon as WithTypeArg<TOut>>::Type
    where
        TFunc: Fn(&TIn1, &TIn2) -> TOut,
    {
        let mut result: Vec<TOut> = Vec::new();
        for x1_val in x1 {
            for x2_val in x2 {
                result.push(f(x1_val, x2_val));
            }
        }
        result
    }
}

impl Monad for VecTypeCon {
    fn bind<TIn, TOut, TFuncArg>(
        x: &<Self as WithTypeArg<TIn>>::Type,
        f: TFuncArg,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        TFuncArg: Fn(&TIn) -> <Self as WithTypeArg<TOut>>::Type {
            let mut result: Vec<TOut> = Vec::new();
            for x_val in x {
                let mut sub_vec = f(x_val);
                result.append(&mut sub_vec);
            }
            result
        }
}
