//You are given an array of integers representing coordinates of obstacles situated on a straight line.
//
//Assume that you are jumping from the point with coordinate 0 to the right. You are allowed only to
// make jumps of the same length represented by some integer.
//
//Find the minimal length of the jump enough to avoid all the obstacles.
pub fn avoidObstacles(inputArray: Vec<i32>) -> i32 {
    for index in 2..std::i32::MAX {
        let is_index_min_step = inputArray.iter().all(|x| (x % index) != 0);
        if is_index_min_step {
            return index;
        }
    }
    0
}
