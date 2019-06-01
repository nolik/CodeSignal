//write a function that returns the sum of numbers that appear in the given inputString
pub fn sumUpNumbers(input_string: String) -> i32 {
    input_string
        .rsplit(|c: char| !c.is_ascii_digit())
        .filter(|str| !str.is_empty())
        .map(|str| str.parse::<i32>().unwrap())
        .sum()
}
