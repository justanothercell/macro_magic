macro_rules! expression {
    ($ident: ident) => {
        print!("{}", stringify!($ident));
    };
    ($lit: literal) => {
        print!("{}", stringify!($lit));
    };
    ( ( $($tt:tt)* ) ) => {
        print!("(");
        expression!($($tt)*);
        print!(")");
    };
    ($func: ident ( $($tt:tt)* )) => {
        print!("{}(", stringify!($func));
        expression!($($tt)*);
        print!(")");
    };
    ($func: ident ( $($tt:tt)* ) $op: tt $($tail:tt)*) => {
        expression!($func ( $($tt)* ));
        print!(" {} ", stringify!($op));
        expression!($($tail)*);
    };
    ($start: tt $op: tt $($tail:tt)*) => {
        expression!($start);
        print!(" {} ", stringify!($op));
        expression!($($tail)*);
    };
}

fn main() {
    // expressions are evaluated right-to-left and do not take operator predecense into account.
    expression!(3 / 4 * 3 / 4 + pi);
    println!();
    expression!((3 / 4) * (3 / 4) + pi);
    println!();
    expression!(((3 / 4) * (3 / 4)) + pi);
    println!();
    expression!(asin(sin(x)));
    println!();
    expression!(magic(sin(x) + cos(x)));
    println!();
    expression!(cos(x * y) + 4);
    println!();
    
    // 3 / 4 * 3 / 4 + pi
    // (3 / 4) * (3 / 4) + pi
    // ((3 / 4) * (3 / 4)) + pi
    // asin(sin(x))
    // magic(sin(x) + cos(x))
    // cos(x * y) + 4
}
