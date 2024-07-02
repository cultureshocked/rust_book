#![allow(unused)]

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

pub struct Guess {
    val: i32,
}

impl Guess {
    pub fn new(val: i32) -> Self {
        if val < 1 || val > 100 {
            panic!("Guess should be between 1-100 inclusive");
        }
        Self {
            val,
        }
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn get_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*; // exposes parent data to this module without needing to super:: prefix every
    // parent type

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another_test() {
    }

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
        
        assert!(larger.can_hold(&smaller)); // assert! panics when expression does not evaluate to
        // true
    }

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

        // we negate the result of the function to make the test pass/assert! to work
        assert!(!smaller.can_hold(&larger));
    }

    // assert_eq! and assert_ne! use the == and != operators under the hood
    // this means that both types in the arguments _must_ implement the PartialEq trait.
    // since debug data is also printed, the data must also implement Debug.
    // both Debug and PartialEq are derivable, using:
    // #[derive(Debug, PartialEq)]
    // in trivial cases
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn not_adds_one() {
        assert_ne!(3, add_two(2));
    }

    // we can add custom messages to a test to indicate more descriptively why it failed.
    #[test]
    fn greeting_contains_name() {
        let result = get_greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain the name; value was {}.", result);
    }

    // we can test to see if a panic! is properly performed by a function
    // this will also be displayed in the test output that this test should be panic!ing
    #[test]
    #[should_panic] // annotation that we are hoping for failure
    fn guess_too_small() {
        let data = Guess::new(0);
    }

    // last but not least, we can use a Result<> to define whether a test fails or passes.
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 { // change 4 to 5 to see the test fail as it returns an Err()
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four")) // this msg will also be printed
            // using dbg! in the test output.
        }
    }
}
