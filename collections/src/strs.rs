pub fn strings() {
    // New empty string
    let mut string = String::new();
    assert_eq!(string.is_empty(), true);

    // String literal, stored in binary as str slice
    let data = "initial contents";

    // Can create String type from string literal
    let s = data.to_string();
    let s2 = String::from(data);
    assert_eq!(s, s2);

    // Strings and str slices are UTF-8 encoded
    println!("Hello in several languages:");
    let hello = String::from("Hello");
    println!("{}, len: {}", hello, hello.len());
    // This len will be 6 because it is 6 bytes long
    let hello = String::from("你好");
    println!("{}, len: {}", hello, hello.len());
    let hello = String::from("Hola");
    println!("{}, len: {}", hello, hello.len());
    let hello = String::from("السلام");
    println!("{}, len: {}", hello, hello.len());
    let hello = String::from("नमस्ते");
    println!("{}, len: {}", hello, hello.len());
    let hello = String::from("Olá");
    println!("{}, len: {}", hello, hello.len());
    let hello = String::from("今日は");
    println!("{}, len: {}", hello, hello.len());
    let hello = String::from("안녕하십니까");
    println!("{}, len: {}", hello, hello.len());
    // First chars in UTF-8 can vary, so normal indexing not allowed,
    // must use slices instead.
    println!(
        "First character of hello in Korean, {}, is 3 bytes",
        &hello[0..3]
    );
    let hello_en = String::from("Hello");
    println!(
        "First char of hello in english: {}, is 1 byte",
        &hello_en[0..1]
    );

    // chars method can make things easier
    // can also use bytes method to get raw bytes
    for c in hello.chars() {
        print!("{} ", c);
    }
    println!("");
    for c in hello_en.chars() {
        print!("{} ", c);
    }
    println!("");

    // Can grow strings
    string.push_str("foo");
    println!("{}", string);
    string.push('-'); // takes only a char
    string.push_str("bar");
    println!("{}", string);

    // + operator
    let concat = s2 + " " + &s; // s2 ownership has changed to concat, s still usable
    println!("{}", concat);

    // format macro
    // does not take ownership of any parameters, unlike +
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);
}
