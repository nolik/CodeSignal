/*In the popular Minesweeper game you have a board with some mines and those cells that don't
contain a mine have a number in it that indicates the total number of mines in the neighboring cells.
Starting off with some arrangement of mines we want to create a Minesweeper game setup.

Example

For

matrix = [[true, false, false],
[false, true, false],
[false, false, false]]

the output should be

minesweeper(matrix) =
[[1, 2, 1],
[2, 1, 1],
[1, 1, 1]]
1*/
pub fn minesweeper(matrix: Vec<Vec<bool>>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    for row_index in 0..matrix.len() {
        let mut vec: Vec<i32> = Vec::new();
        for item_index in 0..matrix[row_index].len() {
            let box_value = check_box_element(&matrix, row_index, item_index);
            vec.push(box_value);
        }
        result.push(vec);
    }
    result
}

fn check_box_element(matrix: &Vec<Vec<bool>>, row_index: usize, item_index: usize) -> i32 {
    let width_len = matrix[0].len() - 1;
    let height_len = matrix.len() - 1;
    let mut box_el = 0;

    if row_index != 0 {
        //top
        if item_index != 0 && matrix[row_index - 1][item_index - 1] {
            box_el += 1;
        }

        if matrix[row_index - 1][item_index] {
            box_el += 1;
        }

        if item_index != width_len && matrix[row_index - 1][item_index + 1] {
            box_el += 1;
        }
    }

    //middle
    if item_index != 0 && matrix[row_index][item_index - 1] {
        box_el += 1;
    }

    if item_index != width_len && matrix[row_index][item_index + 1] {
        box_el += 1;
    }

    //bottom
    if row_index != height_len {
        if item_index != 0 && matrix[row_index + 1][item_index - 1] {
            box_el += 1;
        }
        if matrix[row_index + 1][item_index] {
            box_el += 1;
        }
        if item_index != width_len && matrix[row_index + 1][item_index + 1] {
            box_el += 1;
        }
    }

    box_el
}

/*
Input:

matrix: [[false,false,false],
 `[false,false,false]]

Expected Output:

[[0,0,0],
 [0,0,0]]


Input:

matrix: [[true,false,false,true],
        [false,false,true,false],
        [true,true,false,true]]

Expected Output:

[[0,2,2,1],
 [3,4,3,3],
 [1,2,3,1]]


nput:

matrix: [[true,true,true],
 [true,true,true],
 [true,true,true]]

Expected Output:

[[3,5,3],
 [5,8,5],
 [3,5,3]]


 Input:

matrix: [[true,false],
 [true,false],
 [false,true],
 [false,false],
 [false,false]]

Expected Output:

[[1,2],
 [2,3],
 [2,1],
 [1,1],
 [0,0]]
*/
