macro_rules! add {
    (O,,) => { /*print!("O");*/ };
    (I,,) => { print!("I"); };
    (O, O $($a:ident)*, O $($b:ident)*) => {
        print!("O");
        add!(O, $($a)*, $($b)*);
    };
    (I, O $($a:ident)*, O $($b:ident)*) => {
        print!("I");
        add!(O, $($a)*, $($b)*);
    };
    (O, O $($a:ident)*, I $($b:ident)*) => {
        print!("I");
        add!(O, $($a)*, $($b)*);
    };
    (I, O $($a:ident)*, I $($b:ident)*) => {
        print!("O");
        add!(I, $($a)*, $($b)*);
    };
    (O, I $($a:ident)*, O $($b:ident)*) => {
        print!("I");
        add!(O, $($a)*, $($b)*);
    };
    (I, I $($a:ident)*, O $($b:ident)*) => {
        print!("O");
        add!(I, $($a)*, $($b)*);
    };
    (O, I $($a:ident)*, I $($b:ident)*) => {
        print!("O");
        add!(I, $($a)*, $($b)*);
    };
    (I, I $($a:ident)*, I $($b:ident)*) => {
        print!("I");
        add!(I, $($a)*, $($b)*);
    };
}

macro_rules! eval {
    ($($a:ident)* + $($b:ident)*) => {
        add!(O, $($a)*, $($b)*);
    };
}



fn main() {
    // Evaluates binary expressions (kindof)
    // TO be evaluated by the macrO system, 
    // the biary is written in single digits, reversed.
    
    // using O/I instead of 0/1 because: 
    // "captured metavariables except for `:tt`, `:ident` and `:lifetime` 
    //  cannot be compared tO other tokens"
    
    // 1010 + 0010 = 10 + 2 = 12 = 1100 (0011 or OOII in reversed)
    eval!(O I O I + O I O O);
    println!();
    
    // 01001101 + 11101011 = 77 + 235 = 312 = 100111000 (000111001 or OOO111OO1 in reversed)
    eval!(I O I I O O I O + I I O I O I I I);
    println!();
}
