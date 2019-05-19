extern crate core;

mod task;

fn main() {
    println!(
        "{:?}",
        task::line_encoding::lineEncoding("abbcabb".to_string())
    );
}
