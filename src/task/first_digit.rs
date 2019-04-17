pub fn firstDigit(inputString: String) -> char {
    inputString.chars().find(|c| c.is_numeric()).unwrap()
}
