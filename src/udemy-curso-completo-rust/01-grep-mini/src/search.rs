pub struct SearchMatch {
    pub line_number: i32,
    pub line_text: String,
}

pub fn search_pattern<'a>(text: &'a String, pattern: &String) -> Vec<SearchMatch> {
    let mut lines_matches: Vec<SearchMatch> = Vec::new();

    let mut line_number: i32 = 0;
    for line in text.lines() {
        line_number += 1;
        if line.contains(pattern) {
            let search_match: SearchMatch = SearchMatch {
                line_number: line_number,
                line_text: line.to_string(),
            };
            lines_matches.push(search_match);
        }
    }

    lines_matches
}
