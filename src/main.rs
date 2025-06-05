use std::{env, process};

fn main() {
    let mut args = env::args();
    let action: String;
    let mut params: Vec<String> = Vec::new();

    // Skip binary name argument 
    args.next();

    // NOTE: may eventually make no arguments default to the "list" command
    match args.next() {
        Some(a) => { action = a; },
        None => {
            eprintln!("ERROR: No arguments passed, you must provide an action.");
            process::exit(1);
        }
    }

    for param in args {
        params.push(param);
    }

    todo::run(&action, params);
}
