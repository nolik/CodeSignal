pub fn variableName(name: String) -> bool {
    let first_item_not_digit = !name.chars().nth(0).unwrap().is_digit(10);
    let all_chars_valid = name.chars().all(|chr| chr.is_alphanumeric() || chr == '_');

    first_item_not_digit && all_chars_valid
}
