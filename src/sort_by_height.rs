//Task is to rearrange the people by their heights in a non-descending order without moving the trees.
//For a = [-1, 150, 190, 170, -1, -1, 160, 180], the output should be
//sortByHeight(a) = [-1, 150, 160, 170, -1, -1, 180, 190].
pub fn sort_by_height(input_array: Vec<i32>) -> Vec<i32> {
    let mut tree_indexes = input_array.clone();
    insertion_sort(&mut tree_indexes)
}

fn insertion_sort(list: &mut Vec<i32>) -> Vec<i32> {
    for i in 1..list.len() {
        println!("list[i] {:?}", list[i]);
        if list[i] == -1 { continue; }
        for j in (1..i + 1).rev() {
            println!("internal cycle listJ-1={:?} | listJ={:?}", list[j - 1], list[j]);
            if list[j] == -1 { continue; }
            let previous_index = prev(list, j);
            if list[previous_index] == -1 { continue; }
            if list[previous_index] <= list[j] {
                println!("as list[previous_index]={:?} <= list[j]={:?} break", list[previous_index], list[j]);
                break;
            }
            println!("swap listJ-1={:?} | listJ={:?}", list[previous_index], list[j]);
            list.swap(previous_index, j)
        }
    }
    list.to_vec()
}

fn prev(list: &mut Vec<i32>, index: usize) -> usize {
    for prev_value in (0..index).rev() {
        if list[prev_value] != -1 {
            println!("prev_value {:?}", list[prev_value]);
            return prev_value;
        }
    }
    println!("prev_value in the end of iter {:?}", list[index]);
    return index;
}