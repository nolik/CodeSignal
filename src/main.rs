extern crate core;

mod task;

fn main() {
    println!(
        "{:?}",
        task::mac_adress::isMAC48Address("0-12-34-56-78-9A".to_string())
    );
}
