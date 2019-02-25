/*
You are given an array of integers. On each move you are allowed to increase exactly one of its
element by one. Find the minimal number of moves required to obtain a strictly increasing sequence
from the input.

Example

For inputArray = [1, 1, 1], the output should be
arrayChange(inputArray) = 3.
*/

pub fn arrayChange(mut input_array: Vec<i32>) -> i32 {
    let mut counter = 0;

    for item in 0..input_array.len() - 1 {
        if input_array[item] >= input_array[item + 1] {
            counter += input_array[item] + 1 - input_array[item + 1];
            input_array[item + 1] = input_array[item] + 1;
        }
    }

    counter
}
