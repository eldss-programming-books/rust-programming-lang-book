struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn iter_demo() {
    // General usage
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // lazy init
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // next method directly
    let mut v1_iter = v1.iter();
    println!("Next: {}", v1_iter.next().unwrap());
    println!("Next: {}", v1_iter.next().unwrap());
    println!("Next: {}", v1_iter.next().unwrap());

    // Other methods on iter
    let total: i32 = v1.iter().sum();
    println!("Sum: {}", total);

    let v2: Vec<_> = v1.iter().map(|x| x * x).collect();
    println!("Using map and collect: {:?}", v2);

    let v2 = v1.clone();
    // Cannot use v2 after this call (does not have to be a consuming iterator)
    let v3: Vec<_> = v2.into_iter().filter(|x| x % 2 == 0).collect();
    println!("Using filter with a consuming iter: {:?}", v3);

    // Using our own iterators
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);

    // Crazy use of own iterator
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!(
        "Sum after combining two counters (skipping the first value of the 
    second), multiplying subsequent values together, filtering for 
    products divisible by 3, and summing the result: {}",
        sum
    );
}
