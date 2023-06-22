use crate::format_special_chars::format_special_char::FormatSpecialChars;

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
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> Option<Vec<char>> {
        if *input_char == self.char_to_replace {
            let mut output_chars: Vec<char> = Vec::new();

            if self.next_char_to_insert_is_second {
                output_chars.push(self.char_to_insert_second);
            } else {
                output_chars.push(self.char_to_insert_first);
            }

            self.next_char_to_insert_is_second = !self.next_char_to_insert_is_second;

            Some(output_chars)
        } else {
            None
        }
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

            let mut formatted_chars = formatter.get_formatted_chars(input_char, next_input_char);

            match &mut formatted_chars {
                Some(formatted_chars) => output_chars.append(formatted_chars),
                None => output_chars.push(*input_char),
            }
        }

        let expected_output_text = "“Hello,” said Alice. “Has anyone seen Bob?”";
        let actual_output_text = output_chars.iter().collect::<String>();
        assert_eq!(expected_output_text, actual_output_text);
    }
}
