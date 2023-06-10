use std::fs;

pub fn read_file(file_path: String) -> String {
    let error_message = format!("ERROR:\nThe file could not be read. File: {}.", file_path);
    let file_content = fs::read_to_string(file_path).expect(error_message.as_str());
    file_content
}
