use std::{fs, process};
use crate::text_manipulation::apply_formatters::apply_formatters;
use crate::text_manipulation::formatters::apostrophe_formatter::ApostropheFormatter;
use crate::text_manipulation::formatters::consecutive_chars_to_special_char_formatter::ConsecutiveCharsToSpecialCharFormatter;
use crate::text_manipulation::formatters::format_text_char_by_char::FormatTextCharByChar;
use crate::text_manipulation::formatters::special_char_pair_formatter::SpecialCharPairFormatter;

pub fn convert_file(file_path: &str) -> String {
    let file_contents: String;
    match fs::read_to_string(file_path) {
        Ok(result) => {
            file_contents = result;
        },
        Err(error) => {
            eprintln!("Failed to read input file due to error: {error}");
            process::exit(1);
        }
    }

    let text_formatters = create_text_formatters();

    let converted_text = apply_formatters(&file_contents, text_formatters);

    converted_text
}

// TODO: Selectively include formatters based on config.
fn create_text_formatters() -> Vec<Box<dyn FormatTextCharByChar>> {
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

    let formatters: Vec<Box<dyn FormatTextCharByChar>> = vec![
        Box::new(apostrophe_formatter),
        Box::new(ellipsis_formatter),
        Box::new(em_dash_formatter),
        Box::new(single_quotes_formatter),
        Box::new(double_quotes_formatter),
    ];

    formatters
}
