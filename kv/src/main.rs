use std::any::type_name;
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    println!("Number of arguments: {}", args.len());

    for arg in &args {
        println!("{}", arg);
    }

    if args.is_empty() {
        println!("No arguments provided.");
        return;
    }

    println!("Parsed arg[0]: ");
    let split_args: Vec<&str> = args[0].split(',').collect();
    for arg in &split_args {
        print!("{:?} -> {:?} ", arg, type_of(arg));
    }
    println!("");
}

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}
