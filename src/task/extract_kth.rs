pub fn extractEachKth(inputArray: Vec<i32>, k: i32) -> Vec<i32> {
    let mut result_vec = Vec::new();
    for (i, item) in inputArray.iter().enumerate() {
        if ((i as i32) + 1) % k != 0 {
            result_vec.push(*item)
        }
    }

    result_vec
}
