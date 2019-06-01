extern crate core;

mod task;

fn main() {
    assert_eq!(
        task::sum_up_numbers::sumUpNumbers("2 apples, 12 oranges".to_string()),
        14
    );
}
