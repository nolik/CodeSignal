//Given a rectangular matrix of characters, add a border of asterisks(*) to it.
//
//Example
//
//For
//
//picture = ["abc",
//"ded"]
//
//the output should be
//
//add_border(picture) = ["*****",
//"*abc*",
//"*ded*",
//"*****"]

//Input:
//
//picture: ["aa",
//"**",
//"zz"]
//
//Expected Output:
//
//["****",
//"*aa*",
//"****",
//"*zz*",
//"****"]
pub fn add_border(picture: Vec<String>) -> Vec<String> {
    let mut copy = Vec::new();
    let border_len = picture.iter().max_by(|x, y| x.len().cmp(&y.len())).unwrap().len();
    let border = "*".repeat(border_len + 2);

    copy.push(border.clone());
    for mut str in &picture {
        copy.push("*".to_string() + &str + &"*".to_string());
    }
    copy.push(border);
    copy
}