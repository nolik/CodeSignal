mod task;

fn main() {
    println!(
        "{:?}",
        task::longest_digit_prefix::longestDigitsPrefix("123aa1".to_string())
    );
}
