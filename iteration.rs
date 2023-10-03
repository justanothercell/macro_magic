macro_rules! reverse {
    () => {};
    ($first: tt $($tt:tt)*) => {
        reverse!($($tt)*);
        print!("{}", stringify!($first));
    };
}

macro_rules! zip {
    (,) => {};
    ($a:literal $($stream_a:literal)*, $b:literal $($stream_b:literal)*) => {
        print!("{} ", stringify!( ($a, $b) ));
        zip!($($stream_a)*, $($stream_b)*);
    }
}

macro_rules! zip_simple {
    ($($stream_a:ident)*, $($stream_b:ident)*) => {
        $(
            print!("{} ", stringify!( ($stream_a, $stream_b) ));
        )*
    }
}

macro_rules! row {
    ($_:ident,) => {};
    ($a:ident, $b:ident $($stream_b:ident)*) => {
        print!("{} ", stringify!( ($a, $b) ));
        row!($a, $($stream_b)*);
    }
}

macro_rules! square {
    (,$($_:ident)*) => {};
    ($a:ident $($stream_a:ident)*, $($stream_b:ident)*) => {
        row!($a, $($stream_b)*);
        println!();
        square!($($stream_a)*, $($stream_b)*);
    }
}


fn main() {
    reverse!(0 1 2 3 4 5 6 7 8 9);
    println!();
    zip!(0 1 2 3 4 5 6 7 8 9, 10 11 12 13 14 15 16 17 18 19);
    println!();
    zip_simple!(0 1 2 3 4 5 6 7 8 9, 10 11 12 13 14 15 16 17 18 19);
    println!();
    square!(A B C D E F, G H I J K L);
    println!();
}
