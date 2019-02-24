/*
You are given an array of integers. On each move you are allowed to increase exactly one of its
element by one. Find the minimal number of moves required to obtain a strictly increasing sequence
from the input.

Example

For inputArray = [1, 1, 1], the output should be
arrayChange(inputArray) = 3.
*/
pub fn array_change(input_array: Vec<i32>) -> i32 {
    let mut counter = 0;
    let max = input_array.iter().max().unwrap();
    let index = input_array.iter().position(|element| element == max).unwrap();
    // find max element in array and place index
    //find local max / split array to 2 part / work for counting with right part
    //call function on left part once again
    let mut local_max = max.clone();
    for item in index + 1..input_array.len() {
        local_max += 1;
        counter += dbg!(local_max) - input_array[item];
    }

    if index > 1 {
        let sliced_vec: Vec<_> = input_array[0..index].iter().cloned().collect();
        counter += array_change(sliced_vec);
    }

    counter
}