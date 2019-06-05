use std::collections::HashSet;

pub fn differentSquares(matrix: Vec<Vec<i32>>) -> i32 {
    let mut result = HashSet::new();
    for row in 1..matrix.len() {
        for item in 1..matrix[row].len() {
            let square = Square(
                matrix[row - 1][item - 1],
                matrix[row - 1][item],
                matrix[row][item - 1],
                matrix[row][item],
            );
            result.insert(square);
        }
    }
    result.len() as i32
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Square(i32, i32, i32, i32);
