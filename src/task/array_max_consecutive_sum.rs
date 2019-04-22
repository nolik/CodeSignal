pub fn arrayMaxConsecutiveSum(inputArray: Vec<i32>, k: i32) -> i32 {
    let mut max = std::i32::MIN;

    for index in 0..inputArray.len() as i32 - k + 1 {
        let mut temp = 0;
        for sub in 0..k {
            temp += inputArray[(index + sub) as usize];
        }

        if temp > max {
            max = temp;
        }
    }

    max
}