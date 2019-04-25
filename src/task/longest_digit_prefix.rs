pub fn longestDigitsPrefix(inputString: String) -> String {
    inputString.chars().take_while(|x| x.is_numeric()).collect::<String>()
}
