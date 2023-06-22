pub struct FormatSpecialCharsResult {
    pub formatted_chars: Vec<char>,
    pub run_next_formatter: bool,
}

pub trait FormatSpecialChars {
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> FormatSpecialCharsResult;
}
