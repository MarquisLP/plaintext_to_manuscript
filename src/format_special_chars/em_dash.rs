pub fn format_em_dashes(text: &String) -> String {
    let mut formatted_text = String::from("");

    let text_chars: Vec<char> = text.chars().collect();

    let text_length = text_chars.len();
    let last_char_index: usize;
    if text_length == 0 {
        last_char_index = 0;
    } else {
        last_char_index = text_length - 1;
    }

    let mut consecutive_hyphens_count = 0;
    for (i, character) in text_chars.iter().enumerate() {
        if *character == '-' {
            consecutive_hyphens_count += 1;

            if consecutive_hyphens_count == 2 {
                formatted_text.push('—');
                consecutive_hyphens_count = 0
            } else if i == last_char_index {
                formatted_text.push('-');
            }
        } else {
            if consecutive_hyphens_count == 1 {
                formatted_text.push('-');
            }

            formatted_text.push(*character);

            consecutive_hyphens_count = 0
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

        let output = format_em_dashes(&input);

        assert_eq!("", output);
    }

    #[test]
    fn returns_unaltered_text_when_no_double_hyphen() {
        let input = String::from("Did you get the x-ray gun from my mother-in-law?");

        let output = format_em_dashes(&input);

        assert_eq!("Did you get the x-ray gun from my mother-in-law?", output);
    }

    #[test]
    fn returns_unaltered_text_when_single_hyphen_at_the_end() {
        let input = String::from("Ending with one hyphen-");

        let output = format_em_dashes(&input);

        assert_eq!("Ending with one hyphen-", output);
    }

    #[test]
    fn replaces_double_hyphen_when_it_is_the_only_text() {
        let input = String::from("--");

        let output = format_em_dashes(&input);

        assert_eq!("—", output);
    }

    #[test]
    fn replaces_double_hyphen_when_at_start_of_text() {
        let input = String::from("--Hello there!");

        let output = format_em_dashes(&input);

        assert_eq!("—Hello there!", output);
    }

    #[test]
    fn replaces_double_hyphen_when_at_end_of_text() {
        let input = String::from("Hello there!--");

        let output = format_em_dashes(&input);

        assert_eq!("Hello there!—", output);
    }

    #[test]
    fn replaces_double_hyphen_when_in_the_middle_of_text() {
        let input = String::from("No way--that's impossible!");

        let output = format_em_dashes(&input);

        assert_eq!("No way—that's impossible!", output);
    }

    #[test]
    fn replaces_consecutive_double_hyphens() {
        let input = String::from("There are----two em dashes in a row.");

        let output = format_em_dashes(&input);

        assert_eq!("There are——two em dashes in a row.", output);
    }

    #[test]
    fn replaces_double_hyphens_and_keeps_single_hyphens() {
        let input = String::from("This is great--we can raise awareness of this non-profit with our co-workers--and then send a follow-up.");

        let output = format_em_dashes(&input);

        assert_eq!("This is great—we can raise awareness of this non-profit with our co-workers—and then send a follow-up.", output);
    }
}
