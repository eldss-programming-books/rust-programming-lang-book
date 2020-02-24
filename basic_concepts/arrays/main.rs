fn main() {
    let chars = ['A', 'B', 'C', 'D'];
    println!("chars: {:?}", chars);  // :? is pretty print
    println!("first: {}", chars[0]);

    let ints: [i32; 5] = [1, 2, 3, 4, 5];
    println!("ints: {:?}", ints);

    let five_threes = [3; 5];
    println!("five_threes: {:?}", five_threes);
}