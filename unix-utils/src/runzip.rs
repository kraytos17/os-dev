use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn rle_decode(input: &str) -> String {
    let mut decoded_op = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        let count = if c.is_ascii_digit() {
            let mut count_str = c.to_string();
            while let Some(&next) = chars.peek() {
                if next.is_ascii_digit() {
                    count_str.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            count_str.parse().unwrap_or(1)
        } else {
            1
        };

        if let Some(ch) = chars.next() {
            decoded_op.push_str(&ch.to_string().repeat(count));
        }
    }

    decoded_op
}

#[allow(dead_code)]
pub fn unzip(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let output_path = Path::new(output_file);
    let mut op_file = File::create(output_path)?;
    let mut file = File::open(input_file)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let decoded_buffer = rle_decode(&buffer);
    writeln!(op_file, "{}", decoded_buffer)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    macro_rules! test_decode {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(rle_decode($input), $expected);
            }
        };
    }

    test_decode!(test_rle_decode_repetitive_characters, "3a3b3a", "aaabbbaaa");

    test_decode!(
        test_rle_decode_non_repetitive_characters,
        "1a1b1c1d1e1f",
        "abcdef"
    );

    test_decode!(test_rle_decode_mixed_characters, "2a1b2c1d2e", "aabccdee");

    test_decode!(
        test_rle_decode_with_escape_sequences,
        "1a3\n1b2\t1c",
        "a\n\n\nb\t\tc"
    );

    #[test]
    fn test_unzip_single_file() {
        let input_filename = "input1.txt";
        let input_content = "3a3b3a";
        create_test_file(input_filename, input_content).expect("Failed to create test file");

        let output_filename = "output1.txt";
        unzip(input_filename, output_filename).expect("Failed to unzip file");

        let decoded_output =
            fs::read_to_string(output_filename).expect("Failed to read output file");
        let expected_output = "aaabbbaaa";
        assert_eq!(decoded_output.trim(), expected_output);

        fs::remove_file(input_filename).expect("Failed to remove input file");
        fs::remove_file(output_filename).expect("Failed to remove output file");
    }

    fn create_test_file(filename: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }
}
