pub fn knapsackLight(value1: i32, weight1: i32, value2: i32, weight2: i32, mut maxW: i32) -> i32 {
    let mut result = 0;
    if value1 >= value2 {
        if weight1 <= maxW {
            result += value1;
            maxW -= weight1;
        }

        if weight2 <= maxW {
            result += value2;
        }
    } else {
        if weight2 <= maxW {
            result += value2;
            maxW -= weight2;
        }

        if weight1 <= maxW {
            result += value1;
        }
    }

    result
}
