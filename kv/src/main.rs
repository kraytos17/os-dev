use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

fn load_cmd_table(file_path: &str) -> io::Result<HashMap<String, String>> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);
    let mut cmd_table = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once(',') {
            cmd_table.insert(key.to_string(), value.to_string());
        }
    }

    Ok(cmd_table)
}

fn save_cmd_table(file_path: &str, cmd_table: &HashMap<String, String>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    let mut writer = BufWriter::new(file);

    for (key, value) in cmd_table {
        writeln!(writer, "{},{}", key, value)?;
    }

    Ok(())
}

fn process_command(command: &str, cmd_table: &mut HashMap<String, String>) {
    let split_args: Vec<&str> = command.split(',').collect();
    match split_args.as_slice() {
        ["put", key, value] => {
            cmd_table.insert(key.to_string(), value.to_string());
            println!("Inserted: key {:?} -> value {:?}", key, value);
        }
        ["get", key] => match cmd_table.get(*key) {
            Some(value) => println!("Retrieved: key {:?} -> value {:?}", key, value),
            None => println!("Key {:?} not found", key),
        },
        ["delete", key] => {
            if cmd_table.remove(*key).is_some() {
                println!("Deleted: key {:?}", key);
            } else {
                println!("Key {:?} not found", key);
            }
        }
        ["clear"] => {
            cmd_table.clear();
            println!("Cleared all entries");
        }
        ["all"] => {
            println!("All entries:");
            for (key, value) in cmd_table {
                println!("key {:?} -> value {:?}", key, value);
            }
        }
        _ => eprintln!("Invalid command: {}", command),
    }
}

fn main() -> io::Result<()> {
    let file_path = "db.txt";

    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let mut cmd_table = load_cmd_table(file_path)?;

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

        process_command(command, &mut cmd_table);
    }

    save_cmd_table(file_path, &cmd_table)?;

    Ok(())
}
