use std::io;
//use std::{env, process};

//use crate::rcat::cat;
//use crate::rgrep::grep;
use crate::rzip::zip;

mod rcat;
mod rgrep;
mod runzip;
mod rzip;

//rcat main
// fn main() {
//     let args: Vec<String> = env::args().skip(1).collect();
//     if args.is_empty() {
//         eprintln!("Usage: my_rust_workspace <file1> <file2> ...");
//         std::process::exit(1);
//     }

//     if let Err(e) = cat(&args) {
//         eprintln!("Error: {}", e);
//         std::process::exit(1);
//     }
// }

//rgrep main
// fn main() {
//     // Collect command line arguments
//     let args: Vec<String> = env::args().collect();
//     if args.len() != 3 {
//         eprintln!("Usage: grep <pattern> <filename>");
//         process::exit(1);
//     }

//     let pattern = &args[1];
//     let filename = &args[2];

//     match grep(pattern, filename) {
//         Ok(matches) => {
//             for line in matches {
//                 println!("{}", line);
//             }
//         }
//         Err(e) => {
//             eprintln!("Error: {}", e);
//             process::exit(1);
//         }
//     }
// }

//rzip main
fn main() -> io::Result<()> {
    let filenames = vec![
        String::from("test_data1.txt"),
        String::from("test_data2.txt"),
    ];
    zip(&filenames, "op.txt")
}