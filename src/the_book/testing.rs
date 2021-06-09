//* test macros:
//*             1- assert!(boolean) >> pass if true and fail if false
//*             2- assert_eq!(x,y) >> pass if (x == y) and fail if (x != y)
//*             3- assert_ne!(x,y) >> pass if (x != y) and fail if (x == y)
//*             4- assert_ne!(x,y,"message {elm1} and {elm2}", elm1 , elm2) >> in case of failure the message is printed such like print!()
//*             4- panic!("panic message") >> panics and prints the panic message

//* cargo test [options] :
//*                         single_test >> cargo test it_works  >> will run the it_works test only
//*                         multiple_tests >> cargo test it >> will run all of the tests that starts with "it" word >> [it_works , it_fails]
//*                         ignored_tests >> cargo test -- --ignored >> will run all of the ignored tests only >> [expensive_test]
//*                         ignored_tests >> cargo test -- --ignored >> will run all of the ignored tests only >> [expensive_test]

//*                         specific integration test >> cargo test --test integration_test >> will run only the integration_test

#![allow(dead_code)]
pub fn run() {
    test_function();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn test_function() -> i64 {
    return 4;
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            // panic if the value is out of range !!
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, test_function());
    }

    #[test]
    fn it_fails() {
        assert_eq!(2 + 1, test_function());
    }

    #[test]
    #[ignore] // ignore this test unless intentionally called from cargo test ignored_test
    fn expensive_test() {
        // code that takes an hour to run
    }

    #[test]
    fn make_it_panic() {
        panic!("this is panicked by intention");
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

        assert!(
            larger.can_hold(&smaller),
            "the hight {} > {} and width {} > {} ",
            larger.height,
            smaller.height,
            smaller.width,
            smaller.width
        );
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")] // waiting for a panic to pass with the expected message string
    fn greater_than_100() {
        Guess::new(200);
    }
}
