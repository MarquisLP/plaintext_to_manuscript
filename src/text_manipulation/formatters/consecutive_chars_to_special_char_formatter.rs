use super::format_special_chars::{FormatSpecialChars, FormatSpecialCharsResult};

pub struct ConsecutiveCharsToSpecialCharFormatter {
    char_to_replace: char,
    char_to_insert: char,
    consecutive_max: usize,
    consecutive_count: usize,
}

impl ConsecutiveCharsToSpecialCharFormatter {
    pub fn new(char_to_replace: char, char_to_insert: char, consecutive_max: usize) -> ConsecutiveCharsToSpecialCharFormatter {
        ConsecutiveCharsToSpecialCharFormatter {
            char_to_replace,
            char_to_insert,
            consecutive_max,
            consecutive_count: 0,
        }
    }
}

impl FormatSpecialChars for ConsecutiveCharsToSpecialCharFormatter {
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> FormatSpecialCharsResult {
        let mut output_chars: Vec<char> = Vec::new();
        let mut run_next_formatter = true;

        if *input_char == self.char_to_replace {
            self.consecutive_count += 1;

            if self.consecutive_count == self.consecutive_max {
                output_chars.push(self.char_to_insert);
                self.consecutive_count = 0;
                run_next_formatter = false;
            } else if _next_input_char.is_none() {
                while self.consecutive_count > 1 {
                    output_chars.push(self.char_to_replace);
                    self.consecutive_count -= 1;
                }
                self.consecutive_count = 0;
            } else {
                // This is to prevent the next character, if it isn't a match, from being consumed by this formatter,
                // which would prevent other formatters from acting on that next character.
                run_next_formatter = !_next_input_char.is_some_and(|character| *character == self.char_to_replace);
            }
        } else {
            if self.consecutive_count > 0 {
                while self.consecutive_count > 1 {
                    output_chars.push(self.char_to_replace);
                    self.consecutive_count -= 1;
                }
                self.consecutive_count = 0;
            }
        }

        FormatSpecialCharsResult { formatted_chars: output_chars, run_next_formatter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_consecutive_matching_chars_and_keeps_other_matching_chars() {
        let input_text = String::from("No...this is very strange. Maybe...the schedule was 8th..10th.");
        let input_text_length = input_text.len();
        let mut output_chars: Vec<char> = Vec::new();
        let mut formatter = ConsecutiveCharsToSpecialCharFormatter::new(
            '.',
            '…',
            3,
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

        let expected_output_text = "No…this is very strange. Maybe…the schedule was 8th..10th.";
        let actual_output_text = output_chars.iter().collect::<String>();
        assert_eq!(expected_output_text, actual_output_text);
    }
}
