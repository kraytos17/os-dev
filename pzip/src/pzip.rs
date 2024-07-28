use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Instant;

thread_local! {
    static BUFFER: std::cell::RefCell<Vec<u8>> = std::cell::RefCell::new(Vec::with_capacity(8 * 1024));
}

fn rle_encode(input: &str) -> String {
    let mut output = Vec::with_capacity(input.len());
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let mut count = 1;
        while i + 1 < bytes.len() && bytes[i] == bytes[i + 1] {
            count += 1;
            i += 1;
        }
        output.extend_from_slice(count.to_string().as_bytes());
        output.push(bytes[i]);
        i += 1;
    }

    String::from_utf8(output).expect("Invalid UTF-8 sequence")
}

pub fn pzip(filenames: &[String], output_filename: &str) -> io::Result<()> {
    let output_file = File::create(output_filename)?;
    let mut output_writer = BufWriter::new(output_file);

    let (sender, receiver) = channel::<String>();
    let num_threads = std::cmp::min(filenames.len(), num_cpus::get());
    let mut handles = Vec::with_capacity(num_threads);

    for filename in filenames {
        let sender = sender.clone();
        let filename = filename.clone();
        let handle = thread::spawn(move || {
            process_file(&filename, sender).expect("Failed to process file");
        });
        handles.push(handle);
    }

    drop(sender);

    for handle in handles {
        handle
            .join()
            .expect("Failed to join handles, thread panicked");
    }

    for res in receiver {
        writeln!(output_writer, "{}", res)?;
    }

    Ok(())
}

fn process_file(filename: &str, sender: Sender<String>) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    BUFFER.with(|buf| {
        let mut buf = buf
            .try_borrow_mut()
            .expect("Could not mutably borrow the thread local storage buffer");
        buf.clear();
        reader.read_to_end(&mut buf)?;
        let buffered_str = String::from_utf8_lossy(&buf);
        let encoded_str = rle_encode(&buffered_str);
        sender
            .send(encoded_str)
            .expect("Failed to send RLE-encoded string");

        Ok(())
    })
}

fn generate_test_file(filename: &str, size: usize, pattern: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let pattern_len = pattern.len();
    let mut written = 0;
    while written < size {
        let to_write = if written + pattern_len > size {
            &pattern[..size - written]
        } else {
            pattern
        };
        file.write_all(to_write.as_bytes())?;
        written += to_write.len();
    }
    Ok(())
}

pub fn benchmark() -> io::Result<()> {
    let test_files = vec![
        ("test1.txt", 1024 * 1024, "a"),            // 1 MB, all 'a'
        ("test2.txt", 1024 * 1024, "abc"),          // 1 MB, repeating 'abc'
        ("test3.txt", 2 * 1024 * 1024, "abcdefgh"), // 2 MB, repeating 'abcdefgh'
    ];

    for (filename, size, pattern) in &test_files {
        generate_test_file(filename, *size, pattern)?;
    }

    let filenames: Vec<String> = test_files.iter().map(|(f, _, _)| f.to_string()).collect();
    let output_filename = "output.txt";

    let start = Instant::now();
    pzip(&filenames, output_filename)?;
    let duration = start.elapsed();

    println!("Time taken to compress files: {:?}", duration);

    Ok(())
}
