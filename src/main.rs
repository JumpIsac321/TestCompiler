use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Enter the file name");
        process::exit(1);
    }
    let file_path = &args[1];
    if let Err(e) = test_compiler::run(file_path) {
        eprintln!("Error occured: {e}");
        process::exit(1);
    }
}
