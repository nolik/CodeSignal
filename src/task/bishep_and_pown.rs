pub fn bishopAndPawn(bishop: String, pawn: String) -> bool {
    let mut bishop_chars = bishop.chars();
    let mut pown_chars = pawn.chars();

    let bishop_position = Position(
        bishop_chars.next().unwrap().to_digit(32).unwrap() as i32,
        bishop_chars.next().unwrap().to_digit(10).unwrap() as i32,
    );
    let pawn_position = Position(
        pown_chars.next().unwrap().to_digit(32).unwrap() as i32,
        pown_chars.next().unwrap().to_digit(10).unwrap() as i32,
    );

    let steps_horizontal = dbg!(pawn_position.0 - bishop_position.0);

    let steps_vertical = dbg!(pawn_position.1 - bishop_position.1);

    steps_horizontal.abs() == steps_vertical.abs()
}

#[derive(Debug)]
struct Position(i32, i32);
