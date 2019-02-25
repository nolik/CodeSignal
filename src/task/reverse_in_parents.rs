use std::iter::FromIterator;

//Write a function that reverses characters in (possibly nested) parentheses in the input string.
//
//Input strings will always be well-formed with matching ()s.
//
//Example
//
//For input_string = "(bar)", the output should be
//reverse_in_parentheses(input_string) = "rab";
//For input_string = "foo(bar)baz", the output should be
//reverse_in_parentheses(input_string) = "foorabbaz";
//For input_string = "foo(bar)baz(blim)", the output should be
//reverse_in_parentheses(input_string) = "foorabbazmilb";
//For input_string = "foo(bar(baz))blim", the output should be
//reverse_in_parentheses(input_string) = "foobazrabblim".
//Because "foo(bar(baz))blim" becomes "foo(barzab)blim" and then "foobazrabblim".
pub fn reverseInParentheses(input_string: String) -> String {
    let chars = input_string.chars();
    let mut switcher = 0;
    let mut buffer: Vec<char> = Vec::new();
    let mut result: Vec<char> = Vec::new();

    for item in chars {
        if item == '(' {
            switcher += 1;
            if switcher == 1 {
                continue;
            }
        }

        if item == ')' {
            switcher -= 1;
            if switcher == 0 {
                let temp_result = reverseInParentheses(String::from_iter(buffer.clone()));
                revert_buffer_add_to_result_str(temp_result, &mut result);
                buffer.clear();
                continue;
            }
        }

        if switcher != 0 {
            buffer.push(item);
        } else {
            result.push(item);
        }
    }


    String::from_iter(result)
}


fn revert_buffer_add_to_result_str(mut buffer: String, result: &mut Vec<char>) {
    for item in buffer.chars().rev() {
        result.push(item);
    }
    buffer.clear()
}
