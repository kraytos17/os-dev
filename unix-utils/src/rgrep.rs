use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
pub fn grep(pattern: &str, filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut matched_op = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.contains(pattern) {
            matched_op.push(line);
        }
    }
    Ok(matched_op)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grep_existing_pattern() {
        let filename = "test_data.txt";
        let pattern = "Lorem ipsum dolor";
        let result = grep(pattern, filename).expect("Error in grep function");

        assert!(!result.is_empty(), "Pattern should exist in the file");
    }

    #[test]
    fn test_grep_nonexistent_pattern() {
        let filename = "test_data.txt";
        let pattern = "nonexistent";
        let result = grep(pattern, filename).expect("Error in grep function");

        assert_eq!(result.len(), 0, "Pattern should not exist in the file");
    }

    #[test]
    fn test_grep_empty_file() {
        let filename = "empty_data.txt";
        let pattern = "pattern";
        let result = grep(pattern, filename).expect("Error in grep function");

        assert_eq!(result.len(), 0, "No patterns should be found in an empty file");
    }
}