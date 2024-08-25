use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write};

#[allow(dead_code)]
pub fn reverse_small_files(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let input = BufReader::new(File::open(input_file)?);
    let mut output = BufWriter::new(File::create(output_file)?);
    let mut lines: Vec<String> = input.lines().map_while(Result::ok).collect();
    lines.reverse();

    for line in lines {
        writeln!(output, "{}", line)?;
    }
    output.flush()?;

    Ok(())
}

#[allow(dead_code)]
pub fn reverse_large_files(input_file: &str, output_file: &str) -> std::io::Result<()> {
    let mut input = BufReader::new(File::open(input_file)?);
    let mut output = BufWriter::new(File::create(output_file)?);
    let file_size = input.seek(SeekFrom::End(0))?;
    let mut buffer = Vec::with_capacity(file_size as usize);

    input.seek(SeekFrom::Start(0))?;
    input.read_to_end(&mut buffer)?;

    if let Some(&b'\n') = buffer.last() {
        buffer.pop();
    }

    let reversed_lines: Vec<String> = buffer
        .split(|&c| c == b'\n')
        .rev()
        .map(|line| String::from_utf8_lossy(line).to_string())
        .collect();

    for line in reversed_lines {
        writeln!(output, "{}", line)?;
    }
    output.flush()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::{Read, Write};

    #[test]
    fn test_reverse_small_files() {
        let input_path = "test_input_small.txt";
        let output_path = "test_output_small.txt";

        let mut input_file = File::create(input_path).unwrap();
        input_file.write_all(b"Hello\nWorld\nRust").unwrap();

        reverse_small_files(input_path, output_path).unwrap();

        let mut output_content = String::new();
        let mut output_file = File::open(output_path).unwrap();
        output_file.read_to_string(&mut output_content).unwrap();

        assert_eq!(output_content, "Rust\nWorld\nHello\n");

        fs::remove_file(input_path).unwrap();
        fs::remove_file(output_path).unwrap();
    }

    #[test]
    fn test_reverse_large_files() {
        let input_path = "test_input_large.txt";
        let output_path = "test_output_large.txt";

        let large_input = "Hello\nWorld\nRust\n".repeat(100);

        let mut input_file = File::create(input_path).unwrap();
        input_file.write_all(large_input.as_bytes()).unwrap();

        reverse_large_files(input_path, output_path).unwrap();

        let expected_output = large_input.lines().rev().collect::<Vec<_>>().join("\n") + "\n";
        let mut output_content = String::new();
        File::open(output_path)
            .and_then(|mut file| file.read_to_string(&mut output_content))
            .unwrap();

        assert_eq!(output_content, expected_output);

        fs::remove_file(input_path).unwrap();
        fs::remove_file(output_path).unwrap();
    }
}
