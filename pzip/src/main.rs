use pzip::benchmark;
use std::io;

mod pzip;

fn main() -> io::Result<()> {
    benchmark()?;
    Ok(())
}
