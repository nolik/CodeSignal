mod task;

fn main() {
    println!(
        "{:?}",
        task::string_rearrangment::stringsRearrangement(vec![
            "bec".to_string(),
            "bef".to_string(),
            "bcc".to_string(),
            "bec".to_string(),
            "bbc".to_string(),
            "bdc".to_string()
        ])
    );
}
