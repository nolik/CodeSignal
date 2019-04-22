pub fn arrayMaxConsecutiveSum(inputArray: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut first_sum_item = inputArray[0];
    for int in 0..k {
        sum += inputArray[int as usize];
    }
    let mut max = sum;

    for index in k..inputArray.len() as i32 {
        let current_item = inputArray[index as usize];
        sum += current_item - first_sum_item;
        first_sum_item = inputArray[(index - k + 1) as usize];

        if sum > max {
            max = sum;
        }
    }

    max
}
