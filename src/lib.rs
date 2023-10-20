use std::fs::File;
use std::io::{self, Read};


pub fn read_file_contents(file: &mut File) -> Result<String, io::Error> {
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compress_text(text: &str) -> String {
    let mut compressed_text = String::new();
    let mut prev_char: Option<char> = None;

    for current_char in text.chars() {
        if prev_char != Some(current_char) {
            compressed_text.push(current_char);
        }
        prev_char = Some(current_char);
    }

    compressed_text
}
