mod task;

fn main() {
    println!(
        "{:?}",
        task::string_rearrangment::stringsRearrangement(vec![
            "ab".to_string(),
            "bb".to_string(),
            "aa".to_string()
        ])
    );
}
