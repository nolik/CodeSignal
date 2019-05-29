/* Define a word as a sequence of consecutive English letters.
Find the longest word from the given string. */
pub fn longestWord(text: String) -> String {
    text.split_terminator(|c: char| !c.is_alphabetic())
        .max_by_key(|s| s.len())
        .unwrap()
        .to_owned()
}
