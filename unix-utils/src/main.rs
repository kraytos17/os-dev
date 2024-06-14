use crate::rcat::cat;
use std::env;

mod rcat;
mod rgrep;
mod runzip;
mod rzip;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: my_rust_workspace <file1> <file2> ...");
        std::process::exit(1);
    }

    if let Err(e) = cat(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
