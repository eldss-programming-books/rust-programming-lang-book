mod closures;
mod iters;

fn main() {
    // ===== Closures =====
    let user_val = 33;
    let num = 3;
    closures::generate_workout(user_val, num);

    // Use environment
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // Take ownership of environment
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // can no longer use x
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    println!();

    // ===== Iterators =====
    iters::iter_demo();
}
