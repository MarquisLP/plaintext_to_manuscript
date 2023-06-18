pub fn format_ellipsis(text: &String) -> String {
    let mut formatted_text = String::from("");

    let text_chars: Vec<char> = text.chars().collect();

    let mut consecutive_dots_count = 0;

    let text_length = text_chars.len();
    let last_char_index: usize;
    if text_length == 0 {
        last_char_index = 0;
    } else {
        last_char_index = text_length - 1;
    }

    for (i, character) in text_chars.iter().enumerate() {
        if *character == '.' {
            consecutive_dots_count += 1;

            if consecutive_dots_count == 3 {
                formatted_text.push('…');
                consecutive_dots_count = 0
            } else if i == last_char_index {
                while consecutive_dots_count > 0 {
                    formatted_text.push('.');
                    consecutive_dots_count -= 1;
                }
            }
        } else {
            while consecutive_dots_count > 0 {
                formatted_text.push('.');
                consecutive_dots_count -= 1;
            }

            formatted_text.push(*character);

            consecutive_dots_count = 0
        }
    }

    formatted_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_empty_string_for_empty_input() {
        let input = String::from("");

        let output = format_ellipsis(&input);

        assert_eq!("", output);
    }

    #[test]
    fn returns_unaltered_text_when_no_triple_dots() {
        let input = String::from("There's..only two dots here, Dr. Text");

        let output = format_ellipsis(&input);

        assert_eq!("There's..only two dots here, Dr. Text", output);
    }

    #[test]
    fn returns_unaltered_text_when_single_dot_at_the_end() {
        let input = String::from("Ends with a dot.");

        let output = format_ellipsis(&input);

        assert_eq!("Ends with a dot.", output);
    }

    #[test]
    fn returns_unaltered_text_when_two_dots_at_the_end() {
        let input = String::from("Ends with two dots..");

        let output = format_ellipsis(&input);

        assert_eq!("Ends with two dots..", output);
    }

    #[test]
    fn replaces_three_dots_when_it_is_the_only_text() {
        let input = String::from("...");

        let output = format_ellipsis(&input);

        assert_eq!("…", output);
    }

    #[test]
    fn replaces_three_dots_when_at_start_of_text() {
        let input = String::from("...Not sure.");

        let output = format_ellipsis(&input);

        assert_eq!("…Not sure.", output);
    }

    #[test]
    fn replaces_three_dots_when_at_end_of_text() {
        let input = String::from("I wonder...");

        let output = format_ellipsis(&input);

        assert_eq!("I wonder…", output);
    }

    #[test]
    fn replaces_three_dots_when_in_the_middle_of_text() {
        let input = String::from("What if...that was false?");

        let output = format_ellipsis(&input);

        assert_eq!("What if…that was false?", output);
    }

    #[test]
    fn replaces_six_dots_with_two_ellipsis() {
        let input = String::from("Two......ellipsis");

        let output = format_ellipsis(&input);

        assert_eq!("Two……ellipsis", output);
    }

    #[test]
    fn replaces_three_dots_and_keeps_single_and_double_dots() {
        let input = String::from("No...this is very strange. Maybe...the schedule was 8th..10th.");

        let output = format_ellipsis(&input);

        assert_eq!("No…this is very strange. Maybe…the schedule was 8th..10th.", output);
    }
}
