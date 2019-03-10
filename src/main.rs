mod task;

fn main() {
    //Input:
    //
    /*matrix: [[true,false],
        [true,false],
        [false,true],
        [false,false],
        [false,false]]

    Expected Output:

    [[1,2],
        [2,3],
        [2,1],
        [1,1],
        [0,0]]*/
    println!(
        "{:?}",
        task::minesweeper::minesweeper(vec![
            vec![true, false],
            vec![true, false],
            vec![false, true],
            vec![false, false],
            vec![false, false]
        ])
    );
}
