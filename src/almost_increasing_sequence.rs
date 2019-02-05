pub fn fn almost_increasing_sequence(array: Vec<i32>) -> bool {
    let mut prev = std::i32::MIN;
    let mut before_prev = std::i32::MIN;
    let mut allow_exceptions = true;

    for &item in &array {
        if item <= prev {
            if !allow_exceptions {
                return false;
            }

            allow_exceptions = false;

            if item > before_prev {
                prev = item;
            }
        } else {
            before_prev = prev;
            prev = item;
        }
    }

    true
}