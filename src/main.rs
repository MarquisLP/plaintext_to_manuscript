use std::env;
use plaintext_to_manuscript::convert_file::convert_file;

struct Config {
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let file_path = args[1].clone();

    Config { file_path }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    let converted_text = convert_file(&config.file_path);

    println!("{}", converted_text);
}
