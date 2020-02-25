fn main() {
    let s = String::from("hello world");

    // Note: Can have multiple immutable references in a given scope,
    // but only one mutable reference, and mutable references cannot be 
    // in the same scope as immutable references.
    let hello = &s[..5];  // don't need 0 for start index
    let world = &s[6..];  // don't need 11 for last index if it's the end of s
    
    println!("Full: {}, first word: {}, second word: {}", s, hello, world);
}