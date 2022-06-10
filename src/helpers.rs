pub fn trim_start_end_char(string: &str) -> String {
    String::from(&string[1..string.len() - 1])
}
