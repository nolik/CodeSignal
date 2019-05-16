pub fn isMAC48Address(possible_mac: String) -> bool {
    let splits = possible_mac.split('-').collect::<Vec<&str>>();
    ;

    splits.len() == 6
        && splits
        .into_iter()
        .all(|digit_str| digit_str.len() == 2 && digit_str.chars().all(|x| x.is_digit(16)))
}
