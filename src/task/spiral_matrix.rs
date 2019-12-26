/*
Construct a square matrix with a size N × N containing integers from 1 to N * N in a spiral order,
starting from top-left and in clockwise direction
*/
pub fn spiralNumbers(n: i32) -> Vec<Vec<i32>> {
    let mut result_matrix = vec![vec![0; n as usize]; n as usize];

    let mut starting_row_index = 0;
    let mut starting_column_index = 0;
    let mut ending_row_index = n.clone() as usize;
    let mut ending_column_index = n.clone() as usize;
    let mut insert_number = 1;


    while starting_row_index < ending_row_index
        && starting_column_index < ending_column_index {
        // Init the first row from the remaining rows
        for i in starting_column_index..ending_column_index {
            result_matrix[starting_row_index][i] = insert_number;
            insert_number += 1;
        }
        starting_row_index += 1;

        // Init the last column from the remaining columns
        for i in starting_row_index..ending_row_index {
            result_matrix[i][ending_column_index - 1] = insert_number;
            insert_number += 1;
        }
        ending_column_index -= 1;

        // Init the last row from the remaining rows
        if starting_row_index < ending_row_index {
            for i in (starting_column_index..ending_column_index).rev() {
                result_matrix[ending_row_index - 1][i] = insert_number;
                insert_number += 1;
            }
            ending_row_index -= 1;
        }

//         Init the first column from the remaining columns */
        if starting_column_index < ending_column_index {
            for i in (starting_row_index..ending_row_index).rev() {
                result_matrix[i][starting_column_index] = insert_number;
                insert_number += 1;
            }
            starting_column_index += 1;
        }
    }

    result_matrix
}
