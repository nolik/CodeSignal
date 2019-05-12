use std::ops::Add;

/*
Given a string, find the shortest possible string which can be achieved by adding characters to the
 end of initial string to make it a palindrome.

Example

For st = "abcdc", the output should be
buildPalindrome(st) = "abcdcba".
*/
pub fn buildPalindrome(st: String) -> String {
    if is_palindrome(st.clone()) {
        return st;
    }

    let len = st.len();
    for x in 1..len {
        let (first, _last) = st.split_at(x);
        let reversed = first.chars().rev().collect::<String>();
        let result = String::from(st.clone()).add(&reversed);
        if is_palindrome(result.clone()) {
            return result;
        }
    }
    return st;
}

fn is_palindrome(str: String) -> bool {
    let len = str.len();

    if len % 2 == 0 {
        let (first, last) = str.split_at(len / 2);
        let reversed = last.chars().rev().collect::<String>();
        return first.eq(&reversed);
    } else {
        let (first, last) = str.split_at((len - 1) / 2);
        let (_middle, tail) = last.split_at(1);
        let reversed = &tail.chars().rev().collect::<String>();
        return first.eq(reversed);
    }
}
