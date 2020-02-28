use std::fmt::Display;

// Traits (like interfaces)
pub trait Summary {
    fn summarize(&self) -> String;

    // Can also use default implementations
    // These can also call other functions in the same trait
    // If overridden, the default impl cannot be used
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    // Can simply omit the default method
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Tweet {
    pub fn new(username: String, content: String) -> impl Summary {
        Tweet {
            username,
            content,
            reply: false,
            retweet: false,
        }
    }
}

// Generics
// Same look for generic enums
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

impl<T: Display + PartialOrd> Point<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let num_list = vec![23, 98, -90, 77];
    let char_list = vec!['a', 'r', 'z', '4', 'h'];

    let result = largest(&num_list);
    println!("largest from num: {}", result);
    let result = largest(&char_list);
    println!("largest from chars: {}", result);

    let int_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    println!("int x: {}, float x: {}", int_point.x(), float_point.x());
    int_point.cmp_display();
    println!();

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("Default summary: {}", tweet.summarize_default());

    // Lifetime restrictions
    let string1 = String::from("long string is long");
    let result_wont_work;
    {
        // New scope
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        // Can't use the following var outside of this scope
        result_wont_work = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // string 2 doesn't live long enough to make this print valid
    // println!("The longest string is {}", result_wont_work);
}

// Can define a generic function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Can define a function to take a trait
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Above is syntactic sugar for the following long form: Trait Bound syntax
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize())
}

// Trait Bound syntax is clearer for more complex cases
pub fn notify3<T: Summary>(item1: T, item2: T) {}
// vs
pub fn notify4(item1: impl Summary, item2: impl Summary) {}

// Can also specify multiple traits
// pub fn notify5<T: Summary + Display>(item: T) {}

// For funcs with multiple generics
// fn some_fn<T, U>(t: T, u: U) -> i32
//     where T: Display + Clone, U: Clone + Debug {}

// Lifetime annotations
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
