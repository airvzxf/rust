mod configuration;
use configuration::Configuration;
mod file;
use file::read_file;
mod search;
use search::search_pattern;
use search::SearchMatch;

pub fn run(args: &Vec<String>) {
    let configuration: Configuration = Configuration::new(args);

    let file_content: String = read_file(configuration.file_path);
    println!("File content:\n{}", file_content);

    let lines_matches: Vec<SearchMatch> =
        search_pattern(&file_content, &configuration.text_pattern);
    for line_match in lines_matches.iter() {
        println!(
            "Find in line #{}: {}",
            line_match.line_number, line_match.line_text
        );
    }
}
