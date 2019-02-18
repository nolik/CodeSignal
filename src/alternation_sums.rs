//Several people are standing in a row and need to be divided into two teams. The first person goes into team 1,
// the second goes into team 2, the third goes into team 1 again, the fourth into team 2, and so on.
//
//You are given an array of positive integers - the weights of the people. Return an array of two integers,
// where the first element is the total weight of team 1, and the second element is the total weight
// of team 2 after the division is complete.
//
//Example
//
//For a = [50, 60, 60, 45, 70], the output should be
//alternatingSums(a) = [180, 105].
//
//Input/Output
//
//[execution time limit] 2 seconds (rs)
//
//[input] array.integer a
//
//Guaranteed constraints:
//1 ≤ a.length ≤ 105,
//45 ≤ a[i] ≤ 100.

pub fn alternating_sums(a: Vec<i32>) -> Vec<i32> {
    let mut sum1 = 0;
    let mut sum2 = 0;
    for item in 0..a.len() {
        if (item + 1) % 2 == 0 {
            sum2 += a[item];
        } else {
            sum1 += a[item];
        }
    }
    vec![sum1, sum2]
}