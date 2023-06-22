use super::formatters::format_special_chars::FormatSpecialChars;

pub fn apply_formatters(input_text: &str, mut formatters: Vec<Box<dyn FormatSpecialChars>>) -> String {
    let num_of_formatters = formatters.len();
    if num_of_formatters == 0 {
        return String::from(input_text);
    }

    let input_text_length = input_text.len();

    let input_chars: Vec<char> = input_text.chars().collect();
    let mut output_chars: Vec<char> = Vec::new();
    for i in 0..input_text_length {
        let input_char = input_chars.get(i).unwrap();
        let next_input_char = input_chars.get(i+1);

        let mut formatter_was_run = false;
        let mut formatter_index: usize = 0;
        while !formatter_was_run && (formatter_index < num_of_formatters)  {
            let formatter = formatters.get_mut(formatter_index).unwrap();

            let mut formatted_chars = formatter.get_formatted_chars(input_char, next_input_char);
            match &mut formatted_chars {
                Some(formatted_chars) => {
                    output_chars.append(formatted_chars);
                    formatter_was_run = true;
                },
                None => {}
            }

            formatter_index += 1;
        }

        if !formatter_was_run {
            output_chars.push(*input_char);
        }
    }

    let output_text = output_chars.iter().collect::<String>();
    output_text
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::formatters::apostrophe_formatter::ApostropheFormatter;
    use super::super::formatters::consecutive_chars_to_special_char_formatter::ConsecutiveCharsToSpecialCharFormatter;
    use super::super::formatters::special_char_pair_formatter::SpecialCharPairFormatter;

    #[test]
    fn applies_all_given_formatters_to_input_text() {
        let input_text = "\"Well then...\", said Kim, \"It's been a while, hasn't it?\"\n\"You got that right!\" said Raul. \"It's been so long--I feel like I forgot everything!\"\n\"So much for the so-called 'memory master'.\" Kim laughed.";

        let apostrophe_formatter = ApostropheFormatter::new();
        let em_dash_formatter = ConsecutiveCharsToSpecialCharFormatter::new(
            '-',
            '—',
            2,
        );
        let ellipsis_formatter = ConsecutiveCharsToSpecialCharFormatter::new(
            '.',
            '…',
            3,
        );
        let single_quotes_formatter = SpecialCharPairFormatter::new(
            '\'',
            '‘',
            '’',
        );
        let double_quotes_formatter = SpecialCharPairFormatter::new(
            '"',
            '“',
            '”',
        );

        let formatters: Vec<Box<dyn FormatSpecialChars>> = vec![
            Box::new(apostrophe_formatter),
            Box::new(ellipsis_formatter),
            Box::new(em_dash_formatter),
            Box::new(single_quotes_formatter),
            Box::new(double_quotes_formatter),
        ];

        let expected_output_text = "“Well then…”, said Kim, “It’s been a while, hasn’t it?”\n“You got that right!” said Raul. “It’s been so long—I feel like I forgot everything!”\n“So much for the so-called ‘memory master’.” Kim laughed.";
        let actual_output_text = apply_formatters(input_text, formatters);
        assert_eq!(expected_output_text, actual_output_text);
    }
}
