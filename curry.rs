#![feature(macro_metavar_expr)]

macro_rules! __c_body {
    ($a: expr; @) => {
        $a
    };
    ($a: expr; @ $op: tt $($t: tt)*) => {
        $a $op __c_body!(; $($t)*)
    };
    ($a: expr, $($arg: expr),*; @ $op: tt $($t: tt)*) => {
        $a $op __c_body!($($arg),*; $($t)*)
    };
    ($($arg: expr),*; $f: ident ( $($a: tt)* ) $op: tt $($t: tt)*) => {
        $f ( __c_body!($($arg),*; $($a)*) ) $op __c_body!($($arg),*; $($t)*)
    };
    //($args: ident, ( $($a: tt)* ) $op: tt $($t: tt)*) => {
    //    ( __c_body!($args, $($a)*) ) $op __c_body!($args, $($t)*)
    //};
    //($args: ident, $f: ident ( $( $($a: tt)* ),* )) => {
    //    $f ( $( __c_body!($args, $($a)*) ),* )
    //};
    ($($arg: expr),*; ( $($a: tt)* )) => {
        __c_body!($($arg),*; $($a)*)
    };
    ($($arg: expr),*; $x: tt) => {
        $x
    };
    ($($arg: expr),*; $x: tt $op: tt $($t: tt)*) => {
        $x $op __c_body!($($arg),*; $($t)*)
    };
}

macro_rules! curry {
    ($ident: ident = $($t: tt)*) => {
        macro_rules! a {
            ($$($expr: expr),*) => { {
                __c_body!($$($expr),*; $($t)*)       
            } }
        }
    };
}

fn main() {
    curry!(a = @ + @ * (@ + 7));
    println!("{}", a!(1, 2, 3)); // 1 + 2 * (3 + 7) = 21
}
