#![feature(macro_metavar_expr)]

macro_rules! __c_body {
    ($args: ident,) => {};
    ($args: ident, < $i: tt >) => {
        $args.$i
    };
    ($args: ident, < $i: tt > $op: tt $($t: tt)*) => {
        $args.$i $op __c_body!($args, $($t)*)
    };
    ($args: ident, $f: ident ( $($a: tt)* ) $op: tt $($t: tt)*) => {
        $f ( __c_body!($args, $($a)*) ) $op __c_body!($args, $($t)*)
    };
    //($args: ident, ( $($a: tt)* ) $op: tt $($t: tt)*) => {
    //    ( __c_body!($args, $($a)*) ) $op __c_body!($args, $($t)*)
    //};
    //($args: ident, $f: ident ( $( $($a: tt)* ),* )) => {
    //    $f ( $( __c_body!($args, $($a)*) ),* )
    //};
    ($args: ident, ( $($a: tt)* )) => {
        ( __c_body!($args, $($a)*) )
    };
    ($args: ident, $x: tt) => {
        $x
    };
    ($args: ident, $x: tt $op: tt $($t: tt)*) => {
        $x $op __c_body!($args, $($t)*)
    };
}

macro_rules! curry {
    ($ident: ident = $($t: tt)*) => {
        macro_rules! a {
            ($$($expr: expr),*) => { {
                let args = ($$($expr),*);
                __c_body!(args, $($t)*)       
            } }
        }
    };
}

fn main() {
    curry!(a = <0> + <1> * (<2> + 7));
    println!("{}", a!(1, 2, 3)); // 1 + 2 * (3 + 7) = 21
}
