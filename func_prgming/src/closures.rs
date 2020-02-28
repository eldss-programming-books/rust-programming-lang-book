use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<T, C>
where
    T: Fn(C) -> C,
    C: Eq + Hash + Copy,
{
    calculation: T,
    values: HashMap<C, C>,
}

impl<T, C> Cacher<T, C>
where
    T: Fn(C) -> C,
    C: Eq + Hash + Copy,
{
    fn new(calculation: T) -> Cacher<T, C> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: C) -> C {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // As a closure
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!")
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}
