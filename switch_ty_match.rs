macro_rules! switch_ty_expr {
    ($ain: expr, $( [$($ty: ident)|+]|$a: ident| $expr: expr ),+) => { {
        let ax = $ain;
        match ax {
            $(
                Value::$ty ($a) => $expr,)+
            )+
        }
    } };
}

enum Value {
    Float(f32),
    Int(i32),
    UInt(u32)
}

fn main() {
    switch_ty_expr!(Value::Float(0.4), [Float|Int|UInt] |a| println!("any: {a}"));
    switch_ty_expr!(Value::Float(1.2), [Float] |a| println!("float: {a}"), [Int|UInt] |a| println!("other: {a}"));
    switch_ty_expr!(Value::Int(5), [Float] |a| println!("float: {a}"), [Int|UInt] |a| println!("other: {a}"));
}
