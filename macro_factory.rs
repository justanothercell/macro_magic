macro_rules! times {
    ($name: ident, $x: expr) => {
        macro_rules! $name {
            ($y: expr) => { $x * $y }
        }
    }
}

times!(two, 2);
times!(three, 3);

fn main() {
    println!("{}", two!(4));
    println!("{}", three!(4));
}
