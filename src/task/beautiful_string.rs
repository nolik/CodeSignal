use std::collections::btree_map::BTreeMap;

/*
A string is said to be beautiful if each letter of the alphabet appears at most as many times as
 than the previous letter; ie: b occurs no more times than a; c occurs no more times than b; etc.

Given a string, check whether it is beautiful.

Example

For inputString = "bbbaacdafe", the output should be
isBeautifulString(inputString) = true;

This string contains 3 as, 3 bs, 1 c, 1 d, 1 e, and 1 f (and 0 of every other letter), so since
there aren't any letters that appear more frequently than the previous letter, this string qualifies
as beautiful.

For inputString = "aabbb", the output should be
isBeautifulString(inputString) = false;

Since there are more bs than as, this string is not beautiful.

For inputString = "bbc", the output should be
isBeautifulString(inputString) = false.

Although there are more bs than cs, this string is not beautiful because there are no as, so
therefore there are more bs than as.
*/
pub fn isBeautifulString(inputString: String) -> bool {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    let mut chars_map: BTreeMap<char, i32> = alphabet.chars().map(|x| (x, 0)).collect();

    for x in inputString.chars() {
        let mut counter = chars_map.get(&x).unwrap().to_owned();
        counter += 1;
        chars_map.insert(x, counter);
    }

    let result = chars_map
        .iter()
        .map(|(key, value)| *value)
        .collect::<Vec<i32>>();

    !result.windows(2).any(|item| item[0] < item[1])
}
