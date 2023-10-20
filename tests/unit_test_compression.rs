#[cfg(test)]
mod tests {
    use std::fs::File;

    use compression::*;

    #[test]
    fn test_read_file_contents() {
        use std::io::prelude::*;
        let content = "hello testing is done";
        let file_name = "test_data.txt";

        let mut file = File::create(&file_name).expect("Failed to create file");
        file.write_all(content.as_bytes())
            .expect("Failed to write content to the file");

        let mut file = File::open(&file_name).expect("Failed to open the file");

        let result = read_file_contents(&mut file).expect("Reading error");

        assert_eq!(content, result);
    }

    #[test]
    fn test_compressed_test() {
        let test = "aaabbaazzz";
        let result = "abaz";

        let compressed = compress_text(test);
        assert_eq!(compressed, result);
    }

    #[test]
    fn test_empty_string() {
        let test = "";
        let result = "";

        let compressed = compress_text(test);
        assert_eq!(compressed, result);
    }

    #[test]
    fn test_single_character() {
        let test = "a";
        let result = "a";

        let compressed = compress_text(test);
        assert_eq!(compressed, result);
    }

    #[test]
    fn test_no_consecutive_repeats() {
        let test = "abcde";
        let result = "abcde";

        let compressed = compress_text(test);
        assert_eq!(compressed, result);
    }
}
