use super::format_special_chars::FormatSpecialChars;

fn is_end_of_phrase(character: char) -> bool {
    (character == '.') || (character == '?') || (character == '!')
}

pub struct ApostropheFormatter {
    previous_input_char: char,
}

impl ApostropheFormatter {
    pub fn new() -> ApostropheFormatter {
        ApostropheFormatter {
            previous_input_char: '\n',
        }
    }
}

impl FormatSpecialChars for ApostropheFormatter {
    fn get_formatted_chars(&mut self, input_char: &char, _next_input_char: Option<&char>) -> Option<Vec<char>> {
        let formatted_chars: Option<Vec<char>>;

        // If the ' character is preceded or followed by a space/newline, we assume it is meant to be a single quotation mark instead of an apostrophe.
        if (*input_char != '\'') || (self.previous_input_char == '\n') || (self.previous_input_char == ' ') || _next_input_char.is_none() || _next_input_char.is_some_and(|_next_input_char| (is_end_of_phrase(self.previous_input_char) && *_next_input_char == ' ') || *_next_input_char == '\n') {
            formatted_chars = None;
        } else {
            formatted_chars = Some(vec!('’'));
        }

        self.previous_input_char = *input_char;

        formatted_chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replaces_ticks_that_are_not_preceded_by_newline_and_are_not_followed_by_space_or_newline() {
        let input_text = String::from("'What're you talkin' about?' said Sally. 'It's her birthday!'\n'I don't believe it!' said Carl. 'Where did the year go?'");
        let input_text_length = input_text.len();
        let mut output_chars: Vec<char> = Vec::new();
        let mut formatter = ApostropheFormatter::new();

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

        let expected_output_text = "'What’re you talkin’ about?' said Sally. 'It’s her birthday!'\n'I don’t believe it!' said Carl. 'Where did the year go?'";
        let actual_output_text = output_chars.iter().collect::<String>();
        assert_eq!(expected_output_text, actual_output_text);
    }
}
