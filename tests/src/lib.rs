#[derive(Debug)]
// Sample code to test

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    // Use module defined avobe
    use super::*;


    // Default test
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // Failed test with panic! macro
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Test using assert! macro (receives a boolean)
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    // Negative test using assert! macro (receives a negated boolean)
    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Test using assert_eq! macro (check if its 2 arguments are equals)
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Test using assert_ne! macro (check if its 2 arguments are not equals)
    #[test]
    fn it_does_not_adds_one() {
        assert_ne!(3, add_two(2));
    }

    // Adding custom messages to assert! macro
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Check for panic with should_panic annotation
    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }

    // should_panic can receive an `expected` argument
    // If so, it compares that `expected` is a substring of panicked message
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100_with_expected() {
        Guess::new(200);
    }

    // Using Result<T, E> variants also works
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

}
