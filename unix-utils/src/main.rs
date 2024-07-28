use std::env;
use std::io;
//use std::process;

//use crate::rcat::cat;
//use crate::rgrep::grep;
//use crate::rzip::zip;
use crate::runzip::unzip;

mod rcat;
mod reverse;
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
// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         eprintln!("Usage: {} <input_filename1> [<input_filename2> ...] <output_filename>", args[0]);
//         std::process::exit(1);
//     }

//     let output_filename = &args[args.len() - 1];
//     let input_filenames = &args[1..args.len() - 1];

//     zip(input_filenames, output_filename)?;
//     println!("Files zipped successfully.");

//     Ok(())
// }

//runzip main
fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <input_filename> <output_filename>", args[0]);
        std::process::exit(1);
    }

    let input_filename = &args[0];
    let output_filename = &args[1];

    unzip(input_filename, output_filename)?;
    println!("Files zipped successfully.");

    Ok(())
}
