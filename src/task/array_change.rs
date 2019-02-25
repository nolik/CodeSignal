/*
You are given an array of integers. On each move you are allowed to increase exactly one of its
element by one. Find the minimal number of moves required to obtain a strictly increasing sequence
from the input.

Example

For inputArray = [1, 1, 1], the output should be
arrayChange(inputArray) = 3.
*/

//Expected output: 89510

pub fn arrayChange(input_array: Vec<i32>) -> i32 {
    let mut max = dbg!(input_array.iter().max().unwrap());
    let mut index = dbg!(input_array
        .iter()
        .position(|element| element == max)
        .unwrap());
    let mut tail = input_array.len();
    let mut counter = 0;
    // find max element in array and place index
    //find local max / split array to 2 part / work for counting with right part
    //call function on left part once again

    while tail != 0 {
        println!("counter before incremention = {:?}", counter);
        counter += dbg!(get_count_of_sliced_array(
            &input_array[index + 1..tail],
            max,
        ));
        tail = index.clone();
        if index != 0 {
            max = dbg!(input_array[0..index].iter().max().unwrap());
            index = dbg!(input_array[0..index]
                .iter()
                .position(|element| element == max)
                .unwrap());
        }
    }

    counter
}

fn get_count_of_sliced_array(input_array: &[i32], max: &i32) -> i32 {
    let mut counter = 0;
    let mut local_max = max.clone();
    for item in input_array {
        local_max += 1;
        counter += dbg!(local_max) - dbg!(item);
        println!("counter in cycle = {:?}", counter);
    }
    counter
}
