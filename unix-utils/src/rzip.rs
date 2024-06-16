use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

//naive compression mechanism
fn rle_encode(input: &str) -> String {
    let mut op = String::new();
    let mut chars = input.chars().peekable();
    while let Some(x) = chars.next() {
        let mut count = 1;
        while chars.peek() == Some(&x) {
            chars.next();
            count += 1;
        }
        op.push_str(&count.to_string());
        op.push(x);
    }
    op
}

#[allow(dead_code)]
pub fn zip(filenames: &[String], output_filename: &str) -> std::io::Result<()> {
    let output_path = Path::new(output_filename);
    let mut output_file = File::create(&output_path)?;

    for filename in filenames {
        let mut file = File::open(filename)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        let encoded_buffer = rle_encode(&buffer);
        writeln!(output_file, "{}", encoded_buffer)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    macro_rules! test_encode {
        ($name:ident, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(rle_encode($input), $expected);
            }
        };
    }
    
    fn create_test_file(filename: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn cleanup_test_files(filenames: &[&str]) {
        for filename in filenames {
            if let Err(err) = fs::remove_file(filename) {
                eprintln!("Failed to remove file {}: {}", filename, err);
            }
        }
    }

    test_encode!(test_rle_repetitive_characters, "aaabbbaaa", "3a3b3a");
    test_encode!(test_rle_non_repetitive_characters, "abcdef", "1a1b1c1d1e1f");
    test_encode!(test_rle_mixed_characters, "aabccdee", "2a1b2c1d2e");
    test_encode!(
        test_rle_with_escape_sequences,
        "a\n\n\nb\t\tc",
        "1a3\n1b2\t1c"
    );

    #[test]
    fn test_zip_single_file() {
        let filename = "test_data_single.txt";
        let content = "aaabbbaaa";
        create_test_file(filename, content).expect("Failed to create test file");

        let filenames = vec![filename.to_string()];
        zip(&filenames, "output1.txt").expect("Failed to zip files");

        let encoded_output = fs::read_to_string("output1.txt").expect("Failed to read output file");
        assert_eq!(encoded_output.trim(), "3a3b3a");

        cleanup_test_files(&[filename, "output1.txt"]);
    }

    #[test]
    fn test_zip_multiple_files() {
        let filename1 = "test_data1.txt";
        let content1 = "aaabbb";
        create_test_file(filename1, content1).expect("Failed to create test file 1");

        let filename2 = "test_data2.txt";
        let content2 = "cccaaa";
        create_test_file(filename2, content2).expect("Failed to create test file 2");

        let filenames = vec![filename1.to_string(), filename2.to_string()];
        zip(&filenames, "output2.txt").expect("Failed to zip files");

        let encoded_output = fs::read_to_string("output2.txt").expect("Failed to read output file");
        let expected_output = "3a3b\n3c3a";
        assert_eq!(encoded_output.trim(), expected_output);

        cleanup_test_files(&[filename1, filename2, "output2.txt"]);
    }
}
