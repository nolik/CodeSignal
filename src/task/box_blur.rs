pub fn boxBlur(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for row in 2..image.len() {
        let mut vec: Vec<i32> = Vec::new();
        for item in 2..image[row].len() {
            let box_value = box_value(row, item, &image);
            vec.push(box_value);
        }
        result.push(vec);
    }
    result
}

fn box_value(row: usize, item: usize, image: &Vec<Vec<i32>>) -> i32 {
    let first_raw = raw_sum_value(
        &image[row - 2][item - 2],
        &image[row - 2][item - 1],
        &image[row - 2][item],
    );
    let second_raw = raw_sum_value(
        &image[row - 1][item - 2],
        &image[row - 1][item - 1],
        &image[row - 1][item],
    );
    let third_raw = raw_sum_value(
        &image[row][item - 2],
        &image[row][item - 1],
        &image[row][item],
    );
    (first_raw + second_raw + third_raw) / 9
}

fn raw_sum_value(a: &i32, b: &i32, c: &i32) -> i32 {
    a + b + c
}

//Input:
//
//image: [[1,1,1],
// [1,7,1],
// [1,1,1]]
//
//Expected Output:
//
//[[1]]

//Input:
//
//image: [[0,18,9],
// [27,9,0],
// [81,63,45]]
//
//Expected Output:
//
//[[28]]

//Input:
//
//image: [[36,0,18,9],
// [27,54,9,0],
// [81,63,72,45]]
//
//Expected Output:
//
//[[40,30]]

//Input:
//
//image: [[7,4,0,1],
// [5,6,2,2],
// [6,10,7,8],
// [1,4,2,0]]
//
//Expected Output:
//
//[[5,4],
// [4,4]]

//Input:
//
//image:
// [[36,0,18,9,9,45,27],
// [27,0,54,9,0,63,90],
// [81,63,72,45,18,27,0],
// [0,0,9,81,27,18,45],
// [45,45,27,27,90,81,72],
// [45,18,9,0,9,18,45],
// [27,81,36,63,63,72,81]]
//
//Expected Output:
//
//[[39,30,26,25,31],
// [34,37,35,32,32],
// [38,41,44,46,42],
// [22,24,31,39,45],
// [37,34,36,47,59]]

//Input:
//
//image: [[36,0,18,9,9,45,27],
// [27,0,254,9,0,63,90],
// [81,255,72,45,18,27,0],
// [0,0,9,81,27,18,45],
// [45,45,227,227,90,81,72],
// [45,18,9,255,9,18,45],
// [27,81,36,127,255,72,81]]
//
//Expected Output:
//
//[[82,73,48,25,31],
// [77,80,57,32,32],
// [81,106,88,68,42],
// [44,96,103,89,45],
// [59,113,137,126,80]]
