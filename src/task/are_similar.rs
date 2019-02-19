/*
Two arrays are called similar if one can be obtained from another by swapping at most one pair of elements in one of the arrays.

Given two arrays a and b, check whether they are similar.

Example

For a = [1, 2, 3] and b = [1, 2, 3], the output should be
areSimilar(a, b) = true.

The arrays are equal, no need to swap any elements.

For a = [1, 2, 3] and b = [2, 1, 3], the output should be
areSimilar(a, b) = true.

We can obtain b from a by swapping 2 and 1 in b.

For a = [1, 2, 2] and b = [2, 1, 1], the output should be
areSimilar(a, b) = false.

Any swap of any two elements either in a or in b won't make a and b equal.
*/


pub fn are_similar(a: Vec<i32>, b: Vec<i32>) -> bool {
    let mut counter_of_not_similar_el = 0;
    let mut temp_vec: Vec<usize> = Vec::new();
    for item in 0..a.len() {
        if a[item] != b[item] {
            counter_of_not_similar_el += 1;
            temp_vec.push(item);

            if counter_of_not_similar_el > 2 {
                return false;
            }
        }
    }
    if temp_vec.len() == 2 {
        let first_index = temp_vec[0];
        let second_index = temp_vec[1];
        if a[first_index] != b[second_index] {
            return false;
        }
        if a[second_index] != b[first_index] {
            return false;
        }
    }
    counter_of_not_similar_el < 3
}
