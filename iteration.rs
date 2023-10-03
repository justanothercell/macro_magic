macro_rules! reverse {
    () => {};
    ($first: ident $($stream:ident)*) => {
        reverse!($($stream)*);
        print!("{}", stringify!($first));
    };
}

macro_rules! zip {
    (,) => {};
    ($a:ident $($stream_a:ident)*, $b:ident $($stream_b:ident)*) => {
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
    reverse!(A B C D E F G H I J K L);
    println!();
    println!();
    zip!(A B C D E F, G H I J K L);
    println!();
    println!();
    zip_simple!(A B C D E F, G H I J K L);
    println!();
    println!();
    square!(A B C D E F, G H I J K L);
    println!();
}
