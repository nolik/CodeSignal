use std::collections::HashSet;

pub fn sum_matrix_el(matrix: Vec<Vec<i32>>) {
    println!("{:?}", matrix);
    let mut fallen_indexes = HashSet::new();
    let mut sum = 0;
    for array_number in matrix {
        println!("{:?}", array_number);
        for index in 0..array_number.len() {
            if array_number[index] == 0 {
                fallen_indexes.insert(index);
            }
            if !fallen_indexes.contains(&index) {
                sum += array_number[index];
            }
        }
    }
    println!("{:?}", sum);
}
//
//1 ≤ matrix.length ≤ 5,
//1 ≤ matrix[i].length ≤ 5,
//0 ≤ matrix[i][j] ≤ 10.