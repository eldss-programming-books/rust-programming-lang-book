fn main() {
    one_param(5);
    two_params(-9, 2);

    let mut x = return_five();
    println!("return_five evaluates to {}", x);
    
    x = increment(x);
    println!("now x is {}", x);
    x = increment(x);
    println!("now x is {}", x);
    x = increment(x);
    println!("now x is {}", x);
    x = increment(x);
    println!("now x is {}", x);
}

fn one_param(x: i32) {
    println!("one_param called with {}", x);
}

fn two_params(x: i32, y: i32) {
    println!("two_params called with ({}, {})", x, y);
}

fn return_five() -> i32 {
    // no return keyword, expressions have no ';'
    // Fns are statements unless the last line is an expression,
    // then the function is an expression and evaluates to a value.
    5
}

fn increment(x: i32) -> i32 {
    x + 1
}