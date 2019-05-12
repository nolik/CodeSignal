pub fn chessBoardCellColor(cell1: String, cell2: String) -> bool {
    is_black(cell1.to_uppercase()) == is_black(cell2.to_uppercase())
}

fn is_black(cell: String) -> bool {
    let mut chars = cell.chars();
    let char1 = chars.next().unwrap();
    let char2 = dbg!(chars.next().unwrap().to_digit(10).unwrap());
    match char1 {
        'A' | 'C' | 'E' | 'G' => (char2 + 1) % 2 == 0,
        _ => char2 % 2 == 0,
    }
}
