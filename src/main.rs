extern crate core;
extern crate linked_hash_set;

mod task;

/*
If you are having troubles try:
test:
["a(2)","a","a(3)","a","a","a(2)","a","a(2)","a(1)","a","a","a","a","a","a"]

output:
["a(2)","a","a(3)","a(1)","a(4)","a(2)(1)","a(5)","a(2)(2)","a(1)(1)","a(6)","a(7)","a(8)","a(9)","a(10)","a(11)"]
*/
fn main() {
    assert_eq!(
        task::file_naming::fileNaming(vec![
            "a(2)".to_string(),
            "a".to_string(),
            "a(3)".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a(2)".to_string(),
            "a".to_string(),
            "a(2)".to_string(),
            "a(1)".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string()
        ]),
        vec![
            "a(2)".to_string(),
            "a".to_string(),
            "a(3)".to_string(),
            "a(1)".to_string(),
            "a(4)".to_string(),
            "a(2)(1)".to_string(),
            "a(5)".to_string(),
            "a(2)(2)".to_string(),
            "a(1)(1)".to_string(),
            "a(6)".to_string(),
            "a(7)".to_string(),
            "a(8)".to_string(),
            "a(9)".to_string(),
            "a(10)".to_string(),
            "a(11)".to_string()
        ]
    );
}
