// Standard struct type
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple struct type
struct Color(i32, i32, i32);
struct Point(f64, f64, f64);

fn main() {
    // Create new mutable user
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("User email: {}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("User email: {}", user1.email);

    // Create another user using some of user1's fields
    let user2 = User {
        email: String::from("user2@email.com"),
        username: String::from("user2"),
        // Can do it this way
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,

        // But this is better: Struct update syntax
        ..user1
    };

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn build_user(email: String, username: String) -> User {
    User {
        // Field init shorthand can be used for email and username
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
