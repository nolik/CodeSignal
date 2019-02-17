mod reverse_in_parents;

fn main() {
    println!("{:?}", reverse_in_parents::reverseInParentheses("foo(bar)baz(blim)".to_string()));
}