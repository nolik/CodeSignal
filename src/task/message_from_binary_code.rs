use std::str;
/*
You are taking part in an Escape Room challenge designed specifically for programmers. In your efforts to find a clue, you've found a binary code written on the wall behind a vase, and realized that it must be an encrypted message. After some thought, your first guess is that each consecutive 8 bits of the code stand for the character with the corresponding extended ASCII code.

Assuming that your hunch is correct, decode the message.
*/
pub fn messageFromBinaryCode(code: String) -> String {
    code.as_bytes()
        .chunks(8)
        .map(|e| str::from_utf8(e).unwrap())
        .map(|e| u8::from_str_radix(e, 2).unwrap() as char)
        .collect::<String>()
}
