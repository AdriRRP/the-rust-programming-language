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

    /*
     * By default, tests runs in parallel. To avoid it:
     *   
     *   cargo test -- --test-threads=1
     *
     */


    /*
     * By default, stdout is hidden unless fail. To avoid it:
     *   
     *   cargo test -- --show-output
     *
     */

    /*
     * Running a subset of tests whose names contains 'add'
     *
     *   cargo test add
     *
     */

    /*
     * To ignore tests unless specificall y requested:
     *
     *   #[test]
     *   #[ignore]
     *   fn expensive_test() {
     *     // code that takes an hour to run...
     *   }
     *
     * To request it:
     *
     * cargo test -- --ignored
     *
     */

    /*
     * UNIT TESTS
     *
     * Test each unit of code in isolation from the rest of the code.
     * 
     * Using #[cfg(test)] annotations we avoid to compile test code in final
     * bin.
     *
     * Unit test can test private functions of modules, but we need to import
     * explicitly the module to test.
     *
     * Unit tests go in the same files as the code.
     *
     */

    /*
     * INTEGRATION TESTS
     *
     * Integration tests use your library in the same way any other code would
     * (can only call functions that are part of your library's public API)
     *
     * They're placed in tests directory, at same level that src.
     *
     * Rusts executes a test foreach file in tests directory.
     *
     * If we want to add a common setup for all tests, we need to create 
     * ./tests/common/mod.rs instead of ./tests/common.rs beacause in last 
     * case rust run common.rs as a test.
     *
     */

}
