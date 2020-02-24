fn main() {
    println!("ifs\n----------");
    basic_if();
    exp_in_if();

    println!("loops\n----------");
    loop_kw();
    return_val_from_loop();
    while_kw();
    for_kw();
}

fn basic_if() {
    let num = 3;
    if num < 5 {  // no parens, similar to Go. must be a bool, no truthy vals
        println!("executed true");
    } else {      // else if blocks as expected
        println!("executed false");
    }
}

fn exp_in_if() {
    let condition = true;
    let num = if condition {
        5  // no ';'! this is an expression
    } else {
        6  // must be same type
    }; // now statement is over
    println!("num evaluated to {}", num);
}

fn loop_kw() {
    let mut x = 0;
    loop {
        println!("Can't stop, won't stop (till x=4; x={})", x);
        if x == 4 {
            // this kind of loop needs an explicit break
            break;
        }
        x = x + 1;
    }
}

fn return_val_from_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // expression returned on break
        }
    };
    println!("loop returned {}", result);
}

fn while_kw() {
    let mut num = 3;
    while num != 0 {
        println!("{}...", num);
        num -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_kw() {
    let a = [10, 20, 30, 40, 50];
    println!("Iter through an array.");
    // enumerate with index is optional
    for (index, element) in a.iter().enumerate() {
        println!("Array item {} is {}", index, element);
    }
    
    println!("Iter through range.");
    for num in (1..4).rev() {
        println!("{}...", num);
    }
    println!("LIFTOFF!!!");
}