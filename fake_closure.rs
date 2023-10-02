
macro_rules! fake_closure {
    ($arg1: expr, $arg2: expr, | $a: ident, $b: ident | $expr: expr) => { {
        let $a = $arg1;
        let $b = $arg2;
        let res = $expr;
        res
    } }
}

fn main() {
    println!("{}", fake_closure!(2, 5, |a, b| a * b));
    println!("{}", fake_closure!(7, 3, |a, b| a + b));
}
