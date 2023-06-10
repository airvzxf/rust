pub struct Configuration {
    pub file_path: String,
    pub text_pattern: String,
}

impl Configuration {
    pub fn new(args: &[String]) -> Configuration {
        let file_path: &Option<&String> = &args.get(1);
        println!("file_path: {:#?}", file_path);
        if file_path.is_none() {
            let mut message: String;
            message = "".to_string();
            message += "You need to add the file path and filename as first argument.";
            message += "\n";
            message += "Example: grep-mini something.txt        text_pattern\n";
            message += "Example: grep-mini /home/user/hello.txt text_pattern\n";
            message += "Example: grep-mini ./example/bye.txt    text_pattern\n";
            panic!("ERROR:\n{}", message);
        }

        let text_pattern: &Option<&String> = &args.get(2);
        println!("text_pattern: {:#?}", text_pattern);
        if text_pattern.is_none() {
            let mut message: String;
            message = "".to_string();
            message += "You need to add the text pattern which will be search.";
            message += "\n";
            message += "Example: grep-mini file.txt hello\n";
            message += "Example: grep-mini file.txt \"some words\"\n";
            panic!("ERROR:\n{}", message);
        }

        Configuration {
            file_path: file_path.unwrap().clone(),
            text_pattern: text_pattern.unwrap().clone(),
        }
    }
}
