use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashSet};

pub fn sudoku(grid: Vec<Vec<i32>>) -> bool {
    // check for duplicate numbers in baskets
    let mut first_basket = HashSet::new();
    let mut second_basket = HashSet::new();
    let mut third_basket = HashSet::new();
    let mut column_sets = (0..9)
        .collect::<Vec<usize>>()
        .into_iter()
        .map(|x| (x, HashSet::new()))
        .collect::<BTreeMap<usize, HashSet<&i32>>>();

    for (row_item, row) in grid.iter().enumerate() {
        // check for duplicate numbers on each row
        let mut single_row = HashSet::new();

        if row_item % 3 == 0 {
            first_basket.clear();
            second_basket.clear();
            third_basket.clear();
        }

        for (col_item, col_value) in row.iter().enumerate() {
            if !single_row.insert(col_value) {
                return false;
            }

            // check for duplicate numbers on each column
            let column_set = column_sets.get_mut(&col_item).unwrap();
            if !column_set.insert(col_value) {
                return false;
            };

            // based on col_item put value into basket
            match col_item / 3 {
                0 => {
                    if !first_basket.insert(col_value) {
                        return false;
                    }
                }
                1 => {
                    if !second_basket.insert(col_value) {
                        return false;
                    }
                }
                _ => {
                    if !third_basket.insert(col_value) {
                        return false;
                    }
                }
            }
        }
    }

    true
}
