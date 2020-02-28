mod rectangle; // Additional tests here

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn this_will_panic() {
    panic!("AHHHHH!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Another way to do this with Result<T, E>
    // This way allows use of the ? operator, but have to return Err for test failures
    #[test]
    fn it_works() -> Result<(), String> {
        if add_two(2) == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
    }

    #[test]
    #[ignore] // can run ignored tests with cargo test -- --ignored
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "AHHHHH!")]
    fn panic_at_the_disco() {
        this_will_panic();
    }
}
