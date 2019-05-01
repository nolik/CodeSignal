pub fn bishopAndPawn(bishop: String, pawn: String) -> bool {
    let mut bishop_chars = bishop.chars();
    let mut pown_chars = pawn.chars();

    let bishop_position = dbg!(Position(bishop_chars.next().unwrap(),
                                        bishop_chars.next().unwrap().to_digit(10).unwrap()));
    let pawn_position = dbg!(Position(pown_chars.next().unwrap(),
                                      pown_chars.next().unwrap().to_digit(10).unwrap()));


    true
}

#[derive(Debug)]
struct Position(char, u32);