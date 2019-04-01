/*
Given an array of equal-length strings, you'd like to know if it's possible to rearrange the order
of the elements in such a way that each consecutive pair of strings differ by exactly one character.
 Return true if it's possible, and false if not.

Note: You're only rearranging the order of the strings, not the order of the letters within
the strings!

Example

For inputArray = ["aba", "bbb", "bab"], the output should be
stringsRearrangement(inputArray) = false.

["abc".to_string(),
                "bef".to_string(),
                "bcc".to_string(),
                "bec".to_string(),
                "bbc".to_string(),
                "bdc".to_string()]
                = true

["ff",
 "gf",
 "af",
 "ar",
 "hf"]
*/
pub fn stringsRearrangement(input_array: Vec<String>) -> bool {
    return dbg!(has_one_difference(&input_array[0], &input_array[1]));
}

fn has_one_difference(s1: &str, s2: &str) -> bool {
    // An iterator over different char pairs.
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .take(2)
        .count()
        == 1
}
