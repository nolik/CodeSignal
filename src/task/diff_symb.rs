use std::collections::HashSet;

pub fn differentSymbolsNaive(s: String) -> i32 {
    s.chars().collect::<HashSet<_>>().len() as i32
}
