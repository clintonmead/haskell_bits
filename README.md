# haskell_bits

Various typeclasses and concepts from Haskell, implemented in Rust. ([crates.io link](https://crates.io/crates/haskell_bits))

## Synopsis

Currently this library is an implementation of the Functor/Applicative/Monad hierarchy in Rust. It's very much a work in progress, and I'm releasing it now largely as a request for comment, I'd like to have some discussions about my design choices with others before I go too far down this path.

There has been some work in this space, examples are:

* [`fp-core`](https://crates.io/crates/fp-core)
* [`functional`](https://crates.io/crates/functional)

## So what makes this library different?!

The one thing (I believe) makes this library unique is that one can define generic functions on Monads, Applicatives, Functors etc.

The libraries mentioned above, seem to define Functor, Applicative, Monad for various types, so `fmap`, `bind` etc, work on those types, but, there doesn't seem to be a way to write one generic function that works on all types defined as say, Monad. 

This I think is a very important issue. The great thing about these typeclasses like Monad in Haskell is that if you've got `M` different Monads, and `N` different functions defined on Monads, you've now got effectively `M*N` functions defined. Make a new Monad, you've got a huge infrastucture for free. Make a new Monad function, it works on 100s of Monadic types already defined by others. 

This is one of the key things that makes Monads and associated concepts, and more generally Haskell, so powerful, in that you can combine things in a multiplicative numbers of ways, so much so that many problems just consist of putting together existing bits of code, of which the type system ensures you can only do safely, and almost always, correctly. This is a huge advantage in terms of productivity and in particular, reliability. 


## Example

Here is a simple Haskell function:

```
monadic_pair :: Monad m => m a -> m a -> m (a, a)
monadic_pair x y = do
    x_val <- x
    y_val <- y
    pure (x_val, y_val)
```

And this is the equivalent using this library in Rust:

```
fn monadic_pair<TCon, T, TArg>(x: &TArg, y: &TArg) -> <TCon as WithTypeArg<(T, T)>>::Type
where
    TCon: Monad + WithTypeArg<T> + WithTypeArg<(T, T)>,
    TArg: TypeApp<TCon, T>,
    T: Clone,
{
    mdo! {
        x_val =<< x;
        y_val =<< y;
        ret<TCon> (x_val.clone(), y_val.clone());
    }
}
```

We can then apply this to vectors:

```
let v1: Vec<u32> = vec![1, 2, 3];
let v2: Vec<u32> = vec![4, 5, 6];        
let v_result = monadic_pair(&v1, &v2);

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
```

Or options:

```
let o1: Option<u32> = Some(1);
let o2: Option<u32> = Some(2);
let o3: Option<u32> = None;

let o1_result = monadic_pair(&o1, &o2);
let o2_result = monadic_pair(&o1, &o3);

assert_eq!(o1_result, Some((1, 2)));
assert_eq!(o2_result, None);
```

All with the expected results.

We can even do the standard applicative style function application:

```
let _applicative_option_result = (|x| move |y| x + y).lmap(o1).lap(o2);
assert_eq!(Some(3), _applicative_option_result);
```

With some caveats I'll mention later.

## So how does it work?

Rust does not have higher kinded types, well at least ones we can abstract over. What I mean by this is that whilst Rust happily allows `Vec<T>` for some generic `T`, Rust doesn't allow `T<u32>` for some generic `T`. It allows `Vec<u32>`, but we can't abstract over `Vec` like we can `u32`. 

This has some particular consequences. Lets say we want to write a trait in Rust that encapsulates the idea of applying a function to all parts of a structure. We might try:

```
trait Mapable {
  map<TIn, TOut>(f: impl Func<TIn, TOut>, x: Self<TIn>) -> Self<TOut>;
}

impl Mapable Vec {
  ...
}

impl Mapable Option {
  ...
}

// etc...
```

#### A slight snag

Except we can't do this. Rust will refuse to compile this, because `Vec`, `Option` (and by inference, `Self`) are not types. `Option<u32>` is, but `Option` is not. 

So we need to do a trick:

```
pub trait WithTypeArg<T>
{
    type Type;
}
```

And then:

```
pub struct VecTypeCon;

impl<T> WithTypeArg<T> for VecTypeCon {
    type Type = Vec<T>;
}
```

We can then define `Mapable` like so:

```
pub trait Mapable {
    fn map<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
}
```

And here's a simple implementation for `Option`s:

```
impl Mapable for VecTypeCon {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <OptionTypeCon as WithTypeArg<TIn>>::Type,
    ) -> <OptionTypeCon as WithTypeArg<TOut>>::Type {
        Option::map(x, f) // Option itself already has a specific "map" function
    }
}
```

and for `Vec`s:

```
impl Mapable for VecTypeCon {
    fn map<TIn, TOut>(
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
```

This is pretty good so far, but there's a small issue, which we sort out.

Notice the main motivation for this library is to be able to combine these functions in a generic way.

Well lets make a silly function, that just does two maps in a row (this is a silly function as it would be more efficient just to combine the functions, but lets go with it):

```
fn map2<TCon, TIn, TMid, TOut>(
    f: impl Fn(TIn) -> TMid,
    g: impl Fn(TMid) -> TOut,
    x: <TCon as WithTypeArg<TIn>>::Type,
) -> <TCon as WithTypeArg<TOut>>::Type
{
    map(g, map(f, x))
}
```    

The unfortunately doesn't work, even though it looks quite type correct. I think it's because Rust won't match against a type unless it's fully resolved, and `<TCon as WithTypeArg<TIn>>::Type` can't be resolved because we don't know what `TCon` or `TIn` is. Therefore we can't pass it to `map`, even though it wants an argument of `<TCon as WithTypeArg<TIn>>::Type` (the exact same thing), because I think Rust wants to resolve them both before checking if they're equal.

So what we do is adjust our definition of `WithTypeArg`

```
pub trait WithTypeArg<T> {
    type Type: TypeApp<Self, T>;
}
```

And write this new trait `TypeApp`:

```
pub trait TypeApp<TCon, T>
where
    TCon: WithTypeArg<T>,
    Self: Is<Type = <TCon as WithTypeArg<T>>::Type>,
{
}
```

Yes, notice these traits refer to each other. Also of importance is the `Is` trait. This does take some getting your head around, I would suggest looking at the source of my [`is_type`](https://docs.rs/crate/is_type/0.2.0/source/src/lib.rs) library. But basically what this allows one to do is to force a one-to-one correspondence between a trait and an actual type. 

So you can basically go back and forth between them at will. 

So then we can redefine `map` as follows, replacing any occurance of an argument (but not return type) of:

`<TCon as WithTypeArg<T>>::Type`

with:

`impl TypeApp<TCon, T>`


```
pub trait Mapable {
    fn map<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: impl TypeApp<TCon, TIn>,
    ) -> <Self as WithTypeArg<TOut>>::Type
}
```

and likewise change `map2`:

```
fn map2<TCon, TIn, TMid, TOut>(
    f: impl Fn(TIn) -> TMid,
    g: impl Fn(TMid) -> TOut,
    x: impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
{
    map(g, map(f, x))
}
```

and we're good.

It seems for some reason whilst won't pass `<TCon as WithTypeArg<T>>::Type` to an argument of type `<TCon as WithTypeArg<T>>::Type`, it will pass `<TCon as WithTypeArg<T>>::Type` to a trait argument of `TypeApp<TCon, T>`.

Note in the implementation, that means we'll have to replace all occurances of `x` with `x.into_val()`, where `into_val` is part of the trait `Is` (in the where clause of `TypeApp`).

Basically, this is the core of the approach. We just do a similar thing for Applicative and Monad, but there are a few details and snags that are run into which I'll detail below. 

Note that called the above class `Mappable`, whereas actually in the library what I've detailed above is actually called `LinearFunctor` in the library, which I'll also talk about now:

## The structure of the library.

Currently, there are 7 main traits in the library:

* `Functor`
* `Lift`
* `Applicative`
* `Monad`
* `LinearFunctor`
* `LinearApplicative`
* `LinearMonad`

Functor, Applicative and Monad are basically copies from Haskell, except the `pure` function of `Applicative` is split into the trait `Lift`. So both `Lift` and `Applicative` extend `Functor`, and `Monad` extends both `Lift` and `Applicative`.

The `LinearFunctor/LinearApplicative/LinearMonad` are the "by value" versions of `Functor/Applicative/Monad`. As in they consume their arguments. This can be an efficiency gain because they don't need to copy their arguments, it also means they can be defined on types which aren't `Cloneable`.

The standard prefix for the normal (i.e. by reference versions) is either `f` (in Functor) or no prefix (otherwise), the by value versions (i.e. linear versions) always are prefixed by an `l`. 

Note out of the `Linear*` classes, `LinearFunctor` is a bit special. `Functor` extends `LinearFunctor`, that is every `Functor` is also a `LinearFunctor` (but not the other way around). This is because once one defines `Functor`, you can always define `LinearFunctor` just by taking a reference.

But both `LinearApplicative` and `LinearMonad` take their function argument as an `FnOnce` argument. This is, particularly in the case of `LinearApplicative`, to allow chaining, running a produced closure twice will probably require an explicit `.clone()` at some point. That means `LinearApplicative` and `LinearMonad` can be defined for far less types, generally ones that don't "multiply". Ie. whilst `Vec` is a `LinearFunctor` it's not a `LinearApplicative` or `LinearMonad`, `Option` however is all of these. 

Also all trait functions also have plain old top level functions that call them, as well as often having other trait functions that call them, although these trait functions are only for the purpose of allowing `.` notation. I'll go into some technical details about the reasoning for these now.

## Technical details (particularly for implementors of traits)

I'm going to give the `LinearFunctor` trait as an example. You've seen code similar to this before, but this is the actual code:

```
// Implement this trait for LinearFunctor
pub trait LinearFunctor {
    fn lmap<TIn, TOut>(
        f: impl Fn(TIn) -> TOut,
        x: <Self as WithTypeArg<TIn>>::Type,
    ) -> <Self as WithTypeArg<TOut>>::Type
    where
        Self: WithTypeArg<TIn> + WithTypeArg<TOut>;
}

// Call this for lmap(f, x) syntax
pub fn lmap<TCon, TIn, TOut>(
    f: impl Fn(TIn) -> TOut,
    x: impl TypeApp<TCon, TIn>,
) -> <TCon as WithTypeArg<TOut>>::Type
where
    TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut> + ?Sized,
{
    <TCon as LinearFunctor>::lmap(f, x.into_val())
}

// This is for x.lmapop(f) syntax
pub trait LMapExt {
    fn lmap<TCon, TIn, TOut>(self, x: impl TypeApp<TCon, TIn> + Sized) -> <TCon as WithTypeArg<TOut>>::Type
    where
        Self: Fn(TIn) -> TOut + Sized,
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TOut>
    {
        lmap(self, x)
    }
}

impl<T> LMapExt for T {}
```

Note there's three functions here, one is just a definition, the second is an implementation that calls the first, and the third calls the second.

The first is what you actually implement. Note that the arguments here are actual types. There's no `x: impl TypeApp<TCon, TIn>`. This means, as discussed above, this function [works poorly](#A-slight-snag) when it comes to type inference. 

So the second function here, top level function, instead takes the `TypeApp` trait as it argument, which means it works better with type inference. But then here we need to worry about converting from the trait type to the "real" type, so we call `into_val()` from the `is_type` crate to do this.

The third function is just a trait so we can use the `.` syntax if we want. This becomes useful if we want to chain things like operators, i.e. `f.lmap(x).lap(y)`.

Note in the case of `Functor` only, the function `map` works with both by value and by reference arguments, and calls either `fmap` or `lmap` depending on whether the arguments are by value or by reference.

## Do notation

The macro `!mdo` allows one to write in "do-notation" form. This code is largely stolen (slightly modified) from [rust-mdo](https://github.com/TeXitoi/rust-mdo/blob/master/src/lib.rs).

Note that do notation is currently only defined with the by-reference version of the Monad functions, so you'll see `&` and `Clone::clone(...)` in many places. Not also sometimes `Clone::clone` is better for type inference than `.clone()`, I believe because the latter works for both values and references.

## Quirks

There's a few little quirks one gets from this approch, which makes things slightly messier than equivalent Haskell code in some cases. I'll talk about features that could be added to Rust in [this later section](#Features-that-could-be-added-to-Rust-to-make-things-nicer).

### `WithTypeArg` constraints everywhere

When writing generic functions over Functors, Applicatives or Monads, one has to use the constraint `WithTypeArg` all over the place. Indeed the function `map2` above doesn't actually work, one needs to write it like this:

```
    fn lmap2<TIn, TMid, TOut, TCon>(
        f: impl Fn(TIn) -> TMid,
        g: impl Fn(TMid) -> TOut,
        x: impl TypeApp<TCon, TIn>,
    ) -> <TCon as WithTypeArg<TOut>>::Type
    where
        TCon: LinearFunctor + WithTypeArg<TIn> + WithTypeArg<TMid> + WithTypeArg<TOut>
    {
        lmap(g, lmap(f, x))
    }
```

Notice all the `WithTypeArg<TIn> + WithTypeArg<TMid> + WithTypeArg<TOut>` constraints. One has to put this constraint for every type argument application that even exists in the function. For example in `lmap2` above, we need `WithTypeArg<TMid>` even though it's not an input or output argument, just because it's the result of the innermost map call. 

### Closure types can't be named

In stable rust, closure types can't be named, nor (as far as I could work out) can one define types that implement `Fn` (I think this requires extensions). This comes into play when trying to define some functions. For example, consider the definition of `Applicative`:

```
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
```

We've defined `ap` in terms of `lift2`, but it would be nice to define it the other way around also, so implementers could choose which to implement. Roughly speakking, the [identity](http://hackage.haskell.org/package/base-4.12.0.0/docs/Control-Applicative.html#t:Applicative) is as follows:

```
lift2(f, x, y) = f.fmap(x).ap(y)
```

But in Rust, without automatic currying, we'll need something like:

```
lift2(f, x, y) = (|x| |y| f(x,y)).fmap(x).ap(y)
```

So in this case, the output of `fmap` is an Applicative of Functions. Eventually `ap` applies and this becomes just values, but the intermediate step is a function.

But this function doesn't have a type we can name. But as above, Rust wants a `WithTypeArg` constraint above for every argument to our functions, but we can't write the type for this function because is doesn't have a type name.

A practical issue with this is that you can't create say, and M<F>, where F is some function, inside a function. You have to always pass it in. 

### Some fiddling with type inference

Notice there's actually two `lift()` functions defined (`lift` is just `pure`, but `pure` was a former keyword so I've avoided it).

```
// lift(x)
pub fn lift<TCon, T>(x: T) -> <TCon as WithTypeArg<T>>::Type
where
    <TCon as WithTypeArg<T>>::Type : TypeApp<TCon, T>,
    TCon: Lift + WithTypeArg<T>,
{
    <TCon as Lift>::lift::<T>(x)
}


// lift_c(x)
pub fn lift_c<TCon, T, U>(x: U::Param) -> U
where
    T : Is<Type = U::Param>,
    U : TypeApp<TCon, T>,
    <TCon as WithTypeArg<T>>::Type : TypeApp<TCon, T>,
    TCon: Lift + WithTypeArg<T>,
{
    Is::from_val(lift::<TCon, T>(Is::from_val(x)))
}
```

They both do the same thing, indeed the second just calls the first, but their effect on type inference is different.

The first doesn't allow the inference to go backwards. For example, if one goes:

```
x: Option<u32> = lift(5);
```

then the compiler will not be able to work this out, because `5` could be many different numeric types.

But if one goes like this:

```
x: Option<u32> = lift_c(5);
```

It will be able to work out from the result type that `5` is a `u32`. 

The problem is that `lift_c` doesn't work as well in generic functions over say all monads.

So think of `lift_c` as "lift concrete", as in we have a concrete type.

There's a similar division with `bind` and `bind_c`.

As a result also, there's both `!mdo` and `!mdo_c` macros, `!mdo` should be used in generic code, but `!mdo_c` may be used when one knows the concrete types as there may be better inference in that case.

One example of this is as follows:

```
let o1: Option<u32> = lift_c(5);
let o2: Option<u32> = lift_c(7);

let _do_result: Option<(u32, u32)> = mdo_c! {
    x =<< &o1;
    y =<< &o2;
    ret (Clone::clone(x), Clone::clone(y));
};
```

Notice here one doesn't have to be explicit about the type constructor of the return value out of the macro.

But in `monadic_pair`:

```
    fn monadic_pair<TCon, T, TArg>(x: &TArg, y: &TArg) -> <TCon as WithTypeArg<(T, T)>>::Type
    where
        TCon: Monad + WithTypeArg<T> + WithTypeArg<(T, T)>,
        TArg: TypeApp<TCon, T>,
        T: Clone,
    {
        mdo! {
            x_val =<< x;
            y_val =<< y;
            ret<TCon> (x_val.clone(), y_val.clone());
        }
    }
```

We have to explicitly specify the type constructor of the result of the do-block, namely `TCon`. Rust can't infer this unfortunately.

## Features that could be added to Rust to make things nicer

Overall, despite this looking a bit messy in some places, for client code, it's not too bad. Generic functions over monads are a bit messy to define, but if what you're doing mostly is just using these functions on a particular concrete monad, the code is actually fairly clean. But here's some things that would help:

### Higher kinded types (HKT)

An actual implementation and syntax for higher kinded types would be ideal, as we wouldn't need to do the "hack things into a trait" thing to make this work, and presumably type inference would also be nicer if this was done. But failing that there's a few features which would go a long way to help:

### Forall in where clauses

If we could define something like:

```
trait TypeCon<TCon> where TCon : forall U. WithTypeArg<U> 
```

This would solve the [`WithTypeArg` constraints everywhere](#WithTypeArg-constraints-everywhere) issue. It would also solve the [closure types can't be named](#Closure-types-can't-be-named) issue because they'd be no need to explicitly name such temporary types.

### Generic associated types (GAT)

This is an actual feature underdevelopment which may help, similarly to a "forall" option, but the particular part of this feature needed hasn't landed in nightly yet and has no particular timeline, according to [this issue reply](https://github.com/rust-lang/rust/issues/44265#issuecomment-560941760)

## Future work

There's a few obvious things to be done, firstly implementing Functor/Applicative/Monad for types other than `Option` and `Vec`, `Result` is the obvious next choice, along with other things in the standard Rust library.

`IO` is also a possibility, and I think implementing some of Haskell's [`Parsec`](https://hackage.haskell.org/package/parsec) could allow for some good illistrative examples of the power of this.

Also, some of the more useful monadic functions, like [`mapM`](http://hackage.haskell.org/package/base-4.12.0.0/docs/Control-Monad.html#v:mapM) require [traversable](http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Traversable.html#t:Traversable) and [foldable](http://hackage.haskell.org/package/base-4.12.0.0/docs/Data-Foldable.html#t:Foldable), so these are the obvious traits to implement next.

## Where to start digging around the code.

There's a whole lot of test code dumped in the `test()` function in [`lib.rs`](https://github.com/clintonmead/haskell_bits/blob/master/src/lib.rs), that's probably the best place to start as there's plenty of examples of usage there. 

## Discussion and comments welcome!

The whole point of this was to open up discussion, particularly on core aspects of the design at this point, so please feel free to open up a github issue if you have any suggestions. Or even open up a PR if those changes are more concrete or would just like to add some functions, traits or implementations.


