mod task;

fn main() {
    println!(
        "{:?}",
        task::string_rearrangment::stringsRearrangement(vec![
            "aba".to_string(),
            "bbb".to_string(),
            "bab".to_string()
        ])
    );
}
