pub trait FormatSpecialChars {
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> Option<Vec<char>>;
}
