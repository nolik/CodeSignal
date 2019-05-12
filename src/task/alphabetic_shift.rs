use std::char::from_digit;
use std::string::ToString;

/*
Given a string, replace each of its character by the next one in the English alphabet (z would be replaced by a).

Example

For inputString = "crazy", the output should be
alphabeticShift(inputString) = "dsbaz".
*/
pub fn alphabeticShift(inputString: String) -> String {
    inputString
        .chars()
        .map(|x| {
            let mut digit = x.to_digit(36).unwrap();
            if digit == 35 {
                digit = 10
            } else {
                digit += 1
            }
            return from_digit(digit, 36).unwrap();
        })
        .collect::<String>()
}
