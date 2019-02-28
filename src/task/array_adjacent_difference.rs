pub fn array_maximal_adjacent_difference(input_array: Vec<i32>) -> i32 {
    let mut difference = 0;
    let mut temp_diff = 0;

    for index in 1..input_array.len() {
        if input_array[index - 1] > input_array[index] {
            temp_diff = input_array[index - 1] - input_array[index];
        } else {
            temp_diff = input_array[index] - input_array[index - 1];
        }

        if temp_diff > difference {
            difference = temp_diff;
        }
    }

    difference
}
