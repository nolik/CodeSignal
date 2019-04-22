pub fn arrayMaxConsecutiveSum(inputArray: Vec<i32>, k: i32) -> i32 {
    inputArray.windows(k as usize).map(|window| window.iter().sum()).max().unwrap()
}