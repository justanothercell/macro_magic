macro_rules! idents_equal {
    ($ident: ident, $match: ident) => { {
        macro_rules! matcher {
            ($ident) => { true };
            ($other: ident) => { false };
        }
        matcher!($match)
    } }
}

fn main() {
    println!("{}", idents_equal!(foo, foo)); // true
    println!("{}", idents_equal!(foo, bar)); // false
}
