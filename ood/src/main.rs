mod blog;
mod blog_rust_style;
mod gui;

use gui::{Button, Screen, SelectBox};

fn main() {
    // Polymorphism example
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    // State OO design patter example
    let mut post = blog::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // State OO pattern, but rust style

    // DraftPost
    let mut post = blog_rust_style::Post::new();
    post.add_text("I ate a salad for lunch today");

    // Now a PendingReviewPost
    let post = post.request_review();

    // Final Post; can now see content
    let post = post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
