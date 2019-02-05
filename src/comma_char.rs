pub fn comma_chars(str1: String, mut str2: String) -> u8 {
    let mut counter = 0;
    let first_arr_iterator = str1.chars();

    for character in first_arr_iterator {
        let index = str2.find(character);
        if index != None {
            counter += 1;
            str2.remove(index.unwrap());
        };
        println!("{:?}", character);
    }
    println!("{:?}", counter);
    counter
}