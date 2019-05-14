mod task;

fn main() {
    println!(
        "{:?}",
        task::elections_winners::electionsWinners(vec![2, 3, 5, 2], 3)
    );
}
