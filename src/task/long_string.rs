pub fn calc_long_string(input_array: Vec<String>) -> Vec<String> {
    let limit = input_array.iter().max_by_key(|&x| x.len()).unwrap();
    println!("{:?}", limit);

    input_array.clone()
}
