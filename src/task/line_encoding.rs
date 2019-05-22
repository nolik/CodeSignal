pub fn lineEncoding(line: String) -> String {
    let mut encoded_line: Vec<char> = Vec::new();
    let mut line_chars = line.chars().collect::<Vec<char>>();
    let mut symb = line_chars.remove(0);
    let mut same_chars_counter = 1;
    for x in line_chars {
        if x == symb {
            same_chars_counter += 1;
        } else {
            submit_char(same_chars_counter, symb, encoded_line.as_mut());
            symb = x;
            same_chars_counter = 1;
        }
    }
    submit_char(same_chars_counter, symb, encoded_line.as_mut());

    encoded_line.iter().collect()
}

fn submit_char(char_counter: i32, encoded_char: char, encoded_line: &mut Vec<char>) {
    if char_counter > 1 {
        char_counter
            .to_string()
            .chars()
            .for_each(|parsed_char| encoded_line.push(parsed_char));
    }
    encoded_line.push(encoded_char);
}
