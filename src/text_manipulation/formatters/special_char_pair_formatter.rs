use super::format_special_chars::{FormatSpecialChars, FormatSpecialCharsResult};

pub struct SpecialCharPairFormatter {
    char_to_replace: char,
    char_to_insert_first: char,
    char_to_insert_second: char,
    next_char_to_insert_is_second: bool,
}

impl SpecialCharPairFormatter {
    pub fn new(char_to_replace: char, char_to_insert_first: char, char_to_insert_second: char) -> SpecialCharPairFormatter {
        SpecialCharPairFormatter {
            char_to_replace,
            char_to_insert_first,
            char_to_insert_second,
            next_char_to_insert_is_second: false
        }
    }
}

impl FormatSpecialChars for SpecialCharPairFormatter {
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> FormatSpecialCharsResult {
        let mut formatted_chars: Vec<char> = Vec::new();
        let mut run_next_formatter = true;

        if *input_char == self.char_to_replace {
            if self.next_char_to_insert_is_second {
                formatted_chars.push(self.char_to_insert_second);
            } else {
                formatted_chars.push(self.char_to_insert_first);
            }

            self.next_char_to_insert_is_second = !self.next_char_to_insert_is_second;
            run_next_formatter = false;
        }

        FormatSpecialCharsResult { formatted_chars, run_next_formatter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_matching_chars_with_alternating_first_and_second_special_chars() {
        let input_text = "\"Hello,\" said Alice. \"Has anyone seen Bob?\"";
        let input_text_length = input_text.len();
        let mut output_chars: Vec<char> = Vec::new();

        let mut formatter = SpecialCharPairFormatter::new(
            '"',
            '“',
            '”',
        );


        let input_chars: Vec<char> = input_text.chars().collect();
        for i in 0..input_text_length {
            let input_char = input_chars.get(i).unwrap();
            let next_input_char = input_chars.get(i+1);

            let mut formatter_result = formatter.get_formatted_chars(input_char, next_input_char);

            output_chars.append(&mut formatter_result.formatted_chars);

            if formatter_result.run_next_formatter {
                output_chars.push(*input_char);
            }
        }

        let expected_output_text = "“Hello,” said Alice. “Has anyone seen Bob?”";
        let actual_output_text = output_chars.iter().collect::<String>();
        assert_eq!(expected_output_text, actual_output_text);
    }
}
