extern crate core;

mod task;

fn main() {
    assert_eq!(task::chess_knight::chessKnight("c2".to_string()), 6);
    assert_eq!(task::chess_knight::chessKnight("a1".to_string()), 2);
    assert_eq!(task::chess_knight::chessKnight("d4".to_string()), 8);
}
