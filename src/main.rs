extern crate core;

mod task;

fn main() {
    assert_eq!(
        task::valid_time::validTime(
            "13:58".to_string()
        ),
        true
    );
}
