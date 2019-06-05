extern crate core;

mod task;
/*Given a rectangular matrix containing only digits, calculate the number of different 2 × 2 squares in it.
*/
fn main() {
    assert_eq!(
        task::difference_squares::differentSquares(vec![
            vec![1, 2, 1],
            vec![2, 2, 2],
            vec![2, 2, 2],
            vec![1, 2, 3],
            vec![2, 2, 1]
        ]),
        6
    );
    assert_eq!(
        task::difference_squares::differentSquares(vec![vec![1], vec![2], vec![2]]),
        0
    );
}
