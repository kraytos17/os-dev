use std::{
    io::{self, Write},
    process::{self, Command},
};

const EXIT: &str = "exit";

fn main() {
    let mut input = String::new();

    loop {
        print!("wish> ");
        io::stdout().flush().expect("failed to flush stdout");
        input.clear();

        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read line from stdin");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let tokens = input.split_whitespace().collect::<Vec<_>>();
        if tokens.is_empty() {
            continue;
        }

        match tokens.as_slice() {
            [EXIT] => {
                println!("Exiting shell ...");
                process::exit(0);
            }
            [EXIT, ..] => {
                eprintln!("Error: `exit` should not contain any arguments");
                continue;
            }
            _ => {}
        }

        let mut command = Command::new(tokens[0]);
        if tokens.len() > 1 {
            command.args(&tokens[1..]);
        }

        match command.output() {
            Ok(proc) => {
                println!("Output:\n\n{}", String::from_utf8_lossy(&proc.stdout));

                if !proc.stderr.is_empty() {
                    println!("Error: {}", String::from_utf8_lossy(&proc.stderr));
                }

                if proc.status.success() {
                    println!("Child process completed successfully.");
                } else {
                    println!("Child process failed.");
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command: {}", e);
            }
        }
    }
}
