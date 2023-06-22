use crate::format_special_chars::format_special_char::FormatSpecialChars;

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
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> Option<Vec<char>> {
        let mut output_chars: Vec<char> = Vec::new();

        if *input_char == self.char_to_replace {
            self.consecutive_count += 1;

            if self.consecutive_count == self.consecutive_max {
                output_chars.push(self.char_to_insert);
                self.consecutive_count = 0;
            } else if _next_input_char.is_none() {
                while self.consecutive_count > 0 {
                    output_chars.push(self.char_to_replace);
                    self.consecutive_count -= 1;
                }
            }
        } else {
            if self.consecutive_count > 0 {
                while self.consecutive_count > 0 {
                    output_chars.push(self.char_to_replace);
                    self.consecutive_count -= 1;
                }
                output_chars.push(*input_char);
            } else {
                return None;
            }
        }

        return Some(output_chars);
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

            let mut formatted_chars = formatter.get_formatted_chars(input_char, next_input_char);

            match &mut formatted_chars {
                Some(formatted_chars) => output_chars.append(formatted_chars),
                None => output_chars.push(*input_char),
            }
        }

        let expected_output_text = "No…this is very strange. Maybe…the schedule was 8th..10th.";
        let actual_output_text = output_chars.iter().collect::<String>();
        assert_eq!(expected_output_text, actual_output_text);
    }
}
