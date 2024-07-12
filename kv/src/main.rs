use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

mod command;
mod db;

fn main() -> io::Result<()> {
    let file_path = "db.txt";

    if !(Path::new(file_path).try_exists()?) {
        File::create(file_path)?;
    }

    let mut cmd_table = db::load_db(file_path)?;

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        print!("Enter command: ");
        io::stdout().flush()?;
        stdin.read_line(&mut input)?;

        let command = input.trim();
        if command == "exit" {
            break;
        }

        command::process_command(command, &mut cmd_table);
    }

    db::save_to_db(file_path, &cmd_table)?;

    Ok(())
}
