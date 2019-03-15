mod task;

fn main() {
    println!(
        "{:?}",
        task::chess_board_cell::chessBoardCellColor("H1".to_string(), "H1".to_string())
    );
}
