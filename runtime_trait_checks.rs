use std::fmt::{Debug, Display};

macro_rules! is_trait {
    ($name: ty, $trait: path) => {{
        trait Marker {
            #[allow(unused)]
            fn is_trait() -> bool {
                false
            }
        }
        struct Test<T>(T);
        impl<T: $trait> Test<T> {
            #[allow(unused)]
            fn is_trait() -> bool {
                true
            }
        }
        impl<T> Marker for Test<T>{}
        Test::<$name>::is_trait()
    }}
}

macro_rules! default_or_none {
    ($name: ty) => {{
        trait Defaulter<T> {
            #[allow(unused)]
            fn maybe_default() -> Option<T> {
                None
            }
        }
        struct Temp<T>(T);
        impl<T: Default> Temp<T> {
            #[allow(unused)]
            fn maybe_default() -> Option<T> {
                Some(T::default())
            }
        }
        impl<T> Defaulter<T> for Temp<T>{}
        Temp::<$name>::maybe_default()
    }}
}

macro_rules! run_if_impl {
    ($var: ident: $name: ty, $trait: path => $ret: ty, |$t: ident| $expr: expr) => {{
        trait MaybeImpl<T> {
            #[allow(unused)]
            fn run(_: T) -> Option<$ret> {
                None
            }
        }
        struct Temp<T>(T);
        impl<T: $trait> Temp<T> {
            #[allow(unused)]
            fn run($t: T) -> Option<$ret> {
                Some($expr)
            }
        }
        impl<T> MaybeImpl<T> for Temp<T>{}
        Temp::<$name>::run($var)
    }}
}

#[derive(Debug)]
struct Foo { x: i32 }

fn main() {
    println!("{:?}", is_trait!(i32, Copy));     // true
    println!("{:?}", is_trait!(String, Copy));  // false
    println!("{:?}", default_or_none!(String)); // Some("")
    println!("{:?}", default_or_none!(Foo));    // None
    let f = &Foo { x: 1 };
    println!("{:?}", run_if_impl!(f: &Foo, Debug => String, |x| format!("{x:?}"))); // Some("Foo { x: 1 }")
    println!("{:?}", run_if_impl!(f: &Foo, Display => String, |x| format!("{x}"))); // None
}
