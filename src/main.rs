extern crate core;

mod task;

fn main() {
    assert_eq!(
        task::message_from_binary_code::messageFromBinaryCode(
            "010010000110010101101100011011000110111100100001".to_string()
        ),
        "Hello!".to_string()
    );
}
