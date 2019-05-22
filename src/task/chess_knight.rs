pub fn chessKnight(cell: String) -> i32 {
    let mut resut = 0;
    let mut cell_chars = cell.chars();
    let vertical_position = cell_chars.next().unwrap().to_digit(32).unwrap() as i32 - 9;
    let horizontal_position = cell_chars.next().unwrap().to_digit(10).unwrap() as i32;

    //    top
    if vertical_position + 2 < 9 {
        resut += possible_moves(horizontal_position);
    }

    //    bottom
    if vertical_position - 2 > 0 {
        resut += possible_moves(horizontal_position);
    }

    //    left border
    if horizontal_position + 2 < 9 {
        resut += possible_moves(vertical_position);
    }

    //    right border
    if horizontal_position - 2 > 0 {
        resut += possible_moves(vertical_position);
    }

    resut
}

fn possible_moves(position: i32) -> i32 {
    if position == 1 || position == 8 {
        return 1;
    } else {
        return 2;
    }
}
