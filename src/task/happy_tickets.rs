//For n = 1230, the output should be
//isLucky(n) = true;
//For n = 239017, the output should be
//isLucky(n) = false.

pub fn is_lucky_ticket(n: i32) -> bool {
    let mut digits_vec = number_to_vec(n);
    let vec_len = digits_vec.len();
    if vec_len % 2 > 0 {
        return false;
    }
    let second_substring = digits_vec.split_off(vec_len / 2);
    let first_partition_sum: i32 = digits_vec.iter().sum();
    let second_partition_sum: i32 = second_substring.iter().sum();

    first_partition_sum == second_partition_sum
}


fn number_to_vec(mut digit: i32) -> Vec<i32> {
    let mut digits = Vec::new();
    while digit > 9 {
        digits.push(digit % 10);
        digit = digit / 10;
    }
    digits.push(digit);
    digits.reverse();
    digits
}
