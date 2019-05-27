extern crate core;

mod task;

fn main() {
    assert_eq!(task::delete_digit::deleteDigit(10), 1);
    assert_eq!(task::delete_digit::deleteDigit(109), 19);
    assert_eq!(task::delete_digit::deleteDigit(222250), 22250);
}
