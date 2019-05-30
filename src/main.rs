extern crate core;

mod task;

fn main() {
    //    assert_eq!(task::longest_word::longestWord("Ready[[[, steady, go!".to_string()), "steady".to_string());
    //    assert_eq!(task::longest_word::longestWord("To be or not to be".to_string()), "not".to_string());
    assert_eq!(
        task::longest_word::longestWord(
            "You are the best!!!!!!!!!!!! CodeFighter ever!".to_string()
        ),
        "CodeFighter".to_string()
    );
}
