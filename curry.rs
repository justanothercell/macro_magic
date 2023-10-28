#![feature(macro_metavar_expr)]

macro_rules! __c_body {
    // @
    ($a: expr; @) => {
        $a
    };
    
    // @ $op b
    ($a: expr; @ $op: tt $($t: tt)*) => {
        $a $op __c_body!(; $($t)*)
    };
    // @ $op b
    ($a: expr, $($arg: expr),*; @ $op: tt $($t: tt)*) => {
        $a $op __c_body!($($arg),*; $($t)*)
    };
    
    // f(x)
    ($($arg: expr),*; $f: ident ( $($a: tt)* )) => {
        $f ( __c_body!($($arg),*; $($a)*) )
    };
    // f(x) <op> a
    ($($arg: expr),*; $f: ident ( $($a: tt)* ) $op: tt $($t: tt)*) => {
        $f ( __c_body!($($arg),*; $($a)*) ) $op __c_body!($($arg),*; $($t)*)
    };
    
    // (<x>)
    ($($arg: expr),*; ( $($a: tt)* )) => {
        __c_body!($($arg),*; $($a)*)
    };
    // (<x>) <op> b
    ($($arg: expr),*; ( $($a: tt)* ) $op: tt $($t: tt)*) => {
        ( __c_body!($($arg),*; $($a)*) ) $op __c_body!($($arg),*; $($t)*)
    };
    
    // a
    ($($arg: expr),*; $x: tt) => {
        $x
    };
    // a <op> b
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
