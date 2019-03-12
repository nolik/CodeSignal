/*Check if all digits of the given integer are even.*/
pub fn evenDigitsOnly(n: i32) -> bool {
    let digits: Vec<_> = n.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    digits.iter().all(|&x| x % 2 == 0)
}
