#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn vectors() {
    // Inits
    let v: Vec<i32> = Vec::new();
    assert_eq!(v.is_empty(), true);

    let v2 = vec![1, 2, 3];
    assert_eq!(v2.is_empty(), false);

    // Not very useful unless mutable
    let mut v3 = vec![1, 2, 3];
    v3.push(4);
    v3.push(5);
    v3.push(6);

    // Can access two ways
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);

    match v3.get(5) {
        Some(sixth) => println!("The sixth element is {}", sixth),
        None => println!("There is no sixth element"),
    }

    // Loop
    let v_loop = vec![100, 32, 57, 42];
    // no .iter() like for arrays
    for i in &v_loop {
        print!("{} ", i);
    }
    println!();

    // Loop mutable
    let mut v_loop_mut = vec![100, 32, 57, 42];
    // no .iter() like for arrays
    for i in &mut v_loop_mut {
        *i += 50;
    }
    println!("{:?}", v_loop_mut);

    // Can use enum to store diff types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("{:?}", row);
}
