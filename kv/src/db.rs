use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{self, BufRead, BufReader, BufWriter, Write},
};

pub fn load_db(file_path: &str) -> io::Result<HashMap<u32, String>> {
    let file = OpenOptions::new().read(true).open(file_path)?;
    let reader = BufReader::new(file);

    let mut cmd_table: HashMap<u32, String> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((key, value)) = line.split_once(',') {
            if let Ok(key) = key.parse::<u32>() {
                cmd_table.insert(key, value.to_string());
            } else {
                eprintln!("Invalid key format in file: {}", key);
            }
        }
    }

    Ok(cmd_table)
}

pub fn save_to_db(file_path: &str, cmd_table: &HashMap<u32, String>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    let mut writer = BufWriter::new(file);

    for (key, value) in cmd_table.iter() {
        writeln!(writer, "{},{}", key, value)?;
    }

    Ok(())
}
