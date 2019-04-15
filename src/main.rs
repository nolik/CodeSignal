mod task;

fn main() {
    println!(
        "{:?}",
        task::extract_kth::extractEachKth(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 3)
    );
}
