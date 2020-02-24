fn main() {
    let tup1: (i32, f64, u8, char) = (500, 6.4, 1, 'A');
    println!("tup1: ({}, {}, {}, {})", tup1.0, tup1.1, tup1.2, tup1.3);

    let tup2 = (500, 6.4, 1, 'A'); // new var, default types
    let (w, x, y, z) = tup2; // unpack

    println!("The value of w is: {}", w);
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}