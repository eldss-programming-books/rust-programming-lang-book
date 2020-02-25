// This is an example of the structure of modules in Rust
mod back_of_house;
mod front_of_house;

// Still don't fully understand the use of pub here
// I think, since it is in the lib.rs file, these are now available
// outside of this crate as well
pub use crate::back_of_house::{Appetizer, Breakfast};
pub use crate::front_of_house::hosting;

// Using paths
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn eat_breakfast() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about the type of bread
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // We are not allowed to see the seasonal fruit from here
}

pub fn eat_appetizer() {
    let order1 = Appetizer::Salad;
    let order2 = Appetizer::Soup;
}

// Can get even more specific with use,
// but it is only idomatic to do so for structs/enums/etc
use std::collections::HashMap;

fn example() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// An exception to the above is if there are two structs with
// the same name...
use std::fmt;
use std::io;

fn func1(r: fmt::Result) -> fmt::Result {
    r
}

fn func2(r: io::Result<()>) -> io::Result<()> {
    r
}

// Can also rename a type
use std::fmt::Result;
use std::io::Result as IoResult;

fn func3(r: Result) -> Result {
    r
}

fn func4(r: IoResult<()>) -> IoResult<()> {
    r
}

// Other examples
// use std::io::{self, Write};  // self for std::io
// use std::collections::*;
