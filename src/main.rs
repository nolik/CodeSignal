mod task;

fn main() {
    println!(
        "{:?}",
        task::is_ipv4::isIPv4Address("172.331.254.1".to_string())
    );
}
