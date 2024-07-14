use std::{
    env,
    fs::OpenOptions,
    io::{self, BufRead, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{self, Command, Stdio},
};

enum BuiltinCmd {
    Exit,
    Cd,
    Path,
}

const EXIT: &str = "exit";
const CD: &str = "cd";
const PATH: &str = "path";

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut paths = vec!["/bin".to_string()];

    if args.len() == 2 {
        if let Err(err) = process_file(&args[1], &mut paths) {
            eprintln!("Failed to read file '{}': {}", args[1], err);
            process::exit(1);
        }
    }

    loop {
        print!("wish> ");
        stdout.lock().flush().unwrap_or_else(|err| {
            eprintln!("Failed to flush stdout: {}", err);
            process::exit(1);
        });

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();

        let input = input.trim();
        if !input.is_empty() {
            execute_input(input, &mut paths);
        }
    }
}

fn process_file(file_path: &str, paths: &mut Vec<String>) -> io::Result<()> {
    let file_content = std::fs::read_to_string(file_path)?;
    for line in file_content.lines() {
        execute_input(line, paths);
    }
    Ok(())
}

fn execute_input(input: &str, paths: &mut Vec<String>) {
    let (tokens, output_file) = parse_input(input);
    if tokens.is_empty() {
        return;
    }

    if let Some(builtin_cmd) = check_builtins(&tokens) {
        handle_builtin(builtin_cmd, &tokens, paths);
    } else {
        execute_cmd(&tokens, paths, output_file);
    }
}

fn check_builtins(tokens: &[&str]) -> Option<BuiltinCmd> {
    match tokens[0] {
        EXIT => Some(BuiltinCmd::Exit),
        CD => Some(BuiltinCmd::Cd),
        PATH => Some(BuiltinCmd::Path),
        _ => None,
    }
}

fn handle_builtin(cmd: BuiltinCmd, tokens: &[&str], paths: &mut Vec<String>) {
    match cmd {
        BuiltinCmd::Exit => {
            if tokens.len() > 1 {
                eprintln!("Error: `exit` should not contain any arguments");
            } else {
                println!("Exiting shell ...");
                process::exit(0);
            }
        }
        BuiltinCmd::Cd => {
            if tokens.len() != 2 {
                eprintln!("Usage: cd <directory>");
            } else if let Err(e) = env::set_current_dir(tokens[1]) {
                eprintln!("cd: {}", e);
            }
        }
        BuiltinCmd::Path => {
            *paths = tokens[1..].iter().map(|&s| s.to_string()).collect();
        }
    }
}

fn execute_cmd(tokens: &[&str], paths: &[String], output_file: Option<&str>) {
    let cmd_path = paths
        .iter()
        .map(|path| Path::new(path).join(tokens[0]))
        .find(|cmd_path| is_executable(cmd_path));

    if let Some(cmd_path) = cmd_path {
        let mut command = Command::new(cmd_path);
        if tokens.len() > 1 {
            command.args(&tokens[1..]);
        }

        if let Some(output_file) = output_file {
            let file = match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(output_file)
            {
                Ok(file) => file,
                Err(e) => {
                    eprintln!("Failed to open output file '{}': {}", output_file, e);
                    return;
                }
            };
            command
                .stdin(Stdio::inherit())
                .stdout(Stdio::from(
                    file.try_clone()
                        .expect("Failed to clone file handle for stdout"),
                ))
                .stderr(Stdio::from(file));
        } else {
            command
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit());
        }

        match command.spawn() {
            Ok(mut child) => {
                if let Err(e) = child.wait() {
                    eprintln!("Failed to wait on child process: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Failed to execute command '{}': {}", tokens[0], e);
            }
        }
    } else {
        eprintln!("Command not found: {}", tokens[0]);
    }
}

fn is_executable(path: &Path) -> bool {
    path.exists()
        && path.is_file()
        && path
            .metadata()
            .map(|m| m.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
}

fn parse_input(input: &str) -> (Vec<&str>, Option<&str>) {
    let mut tokens = vec![];
    let mut output_file = None;
    let mut iter = input.split_whitespace().peekable();

    while let Some(part) = iter.next() {
        if part == ">" {
            if let Some(&next_part) = iter.peek() {
                output_file = Some(next_part);
                iter.next();
            } else {
                eprintln!("Syntax error: expected filename after '>'");
                break;
            }
        } else {
            tokens.push(part);
        }
    }

    (tokens, output_file)
}
