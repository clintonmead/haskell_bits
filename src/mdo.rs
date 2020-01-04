// Largely stolen from: 
// https://github.com/TeXitoi/rust-mdo/blob/master/src/lib.rs

#[macro_export]
macro_rules! mdo {
    (
        let $p: pat = $e: expr ; $( $t: tt )*
    ) => (
        { let $p = $e ; mdo! { $( $t )* } }
    );

    (
        let $p: ident : $ty: ty = $e: expr ; $( $t: tt )*
    ) => (
        { let $p: $ty = $e ; mdo! { $( $t )* } }
    );

    (
        $p: pat =<< $e: expr ; $( $t: tt )*
    ) => (
        bind($e, move |$p : &_| mdo! { $( $t )* } )
    );

    (
        $p: ident : $ty: ty =<< $e: expr ; $( $t: tt )*
    ) => (
        bind($e, move |$p : &$ty| mdo! { $( $t )* } )
    );

    (
        ign $e: expr ; $( $t: tt )*
    ) => (
        bind($e, move |_| mdo! { $( $t )* })
    );

    (
        ret<$ty: ty> $e: expr ;
    ) => (
        lift::<$ty, _>($e)
    );
}

#[macro_export]
macro_rules! mdo_c {
    (
        let $p: pat = $e: expr ; $( $t: tt )*
    ) => (
        { let $p = $e ; mdo_c! { $( $t )* } }
    );

    (
        let $p: ident : $ty: ty = $e: expr ; $( $t: tt )*
    ) => (
        { let $p: $ty = $e ; mdo_c! { $( $t )* } }
    );

    (
        $p: pat =<< $e: expr ; $( $t: tt )*
    ) => (
        bind_c($e, move |$p : &_| mdo_c! { $( $t )* } )
    );

    (
        $p: ident : $ty: ty =<< $e: expr ; $( $t: tt )*
    ) => (
        bind_c($e, move |$p : &$ty| mdo_c! { $( $t )* } )
    );

    (
        ign $e: expr ; $( $t: tt )*
    ) => (
        bind_c($e, move |_| mdo_c! { $( $t )* })
    );

    (
        ret<$ty: ty> $e: expr ;
    ) => (
        lift::<$ty, _>($e)
    );

    (
        ret $e: expr ;
    ) => (
        lift_c($e)
    );
}

