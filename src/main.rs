use std::env;

fn main() {
    let mut args = env::args();
    let action: String;
    let mut params: Vec<String> = Vec::new();

    // Skip binary name argument 
    args.next();

    match args.next() {
        Some(a) => { action = a; },
        None => {
            // Make listing the todos the default action
            action = "list".to_string();
        }
    }

    for param in args {
        params.push(param);
    }

    todo::run(&action, params);
}
