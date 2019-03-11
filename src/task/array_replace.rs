pub fn arrayReplace(mut inputArray: Vec<i32>, elemToReplace: i32, mut substitutionElem: i32) -> Vec<i32> {
    for mut elem in inputArray.iter_mut() {
        if *elem == elemToReplace {
            *elem = substitutionElem;
        }
    }
    inputArray
}