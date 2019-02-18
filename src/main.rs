mod border;

fn main() {
    println!("{:?}", border::add_border(vec!["aa".to_string(),
                                             "**".to_string(),
                                             "zz".to_string()]));
}