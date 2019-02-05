pub fn calc_long_string(inputArray: Vec<String>) -> Vec<String> {
//    let mut iter = array.split(|item| item.to_string().len() == 3).collect();
    let limit = inputArray.iter().max_by_key(|&x| x.len()).unwrap();
    println!("{:?}", limit);
//    let iter = inputArray
//        .into_iter()
//        .filter(|str| str.len() == limit.len())
//        .collect();

//    println!("{:?}", iter);
//    iter
    inputArray.clone()
}


//
//For inputArray = ["aba", "aa", "ad", "vcd", "aba"], the output should be
//allLongestStrings(inputArray) = ["aba", "vcd", "aba"].