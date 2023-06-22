pub struct FormatTextCharByCharResult {
    pub formatted_chars: Vec<char>,
    pub run_next_formatter: bool,
}

pub trait FormatTextCharByChar {
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> FormatTextCharByCharResult;
}
