use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;
use std::path::PathBuf;
use std::{fs, io, process};

const DATA_FILE_NAME: &'static str = "todo.dat";

#[derive(Serialize, Deserialize)]
struct Todo {
    label: String,
    complete: bool,
}

#[derive(Serialize, Deserialize)]
struct Settings {
    silent: String,
}

/// Run the todo app.
/// @param action - The action string chosen by the user.
/// @param params - Any parameters passed after the action.
pub fn run(action: &String, params: Vec<String>) {
    let mut settings = extract_settings();
    let (data_path, mut todo_data) = read_to_vec(dirs::data_dir());
    match action.as_str() {
        "add" => {
            add_items(&mut todo_data, params, &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "list" => print_list(&todo_data),
        "remove" => {
            remove_items(&mut todo_data, params, &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "clear" => {
            remove_items(&mut todo_data, vec!["all".to_string()], &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "check" => {
            check_items(&mut todo_data, params, &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "uncheck" => {
            uncheck_items(&mut todo_data, params, &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "sort" => {
            sort_items(&mut todo_data, params, &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "set" => set_setting(&mut settings, params),
        "edit" => {
            edit_item(&mut todo_data, params, &data_path);
            if settings.silent == "off" {
                print_list(&todo_data);
            }
        }
        "help" => show_help(),
        _ => println!("Invalid action: {action}"),
    }
}

/// Read the data file from disk and convert the String data into a String Vector.
/// The output is a tuple where the first element is the finalized data file path
/// and the second element is the data Vector.
/// @param dir - An Option<PathBuf>, where the PathBuf points to the parent directory of the
/// "todo-app" folder that contains the data file.
fn read_to_vec(dir: Option<PathBuf>) -> (String, Vec<Todo>) {
    let mut data: Vec<Todo> = Vec::new();

    let mut path_buf: PathBuf = dir.unwrap_or_else(|| {
        eprintln!("ERROR: Cannot open data directory.");
        process::exit(1);
    });

    path_buf.push("todo-app");

    if let Err(e) = fs::create_dir_all(&path_buf) {
        eprintln!(
            "ERROR: Could not create the data directory at {}: {e}",
            path_buf.to_str().unwrap()
        );
        process::exit(1);
    }

    path_buf.push(DATA_FILE_NAME);

    if let Ok(str) = fs::read_to_string(&path_buf) {
        for line in str.lines() {
            let todo = serde_json::from_str(line).unwrap_or_else(|err| {
                eprintln!("ERROR: Could not parse line \"{line}\" in data file: {err}");
                process::exit(1);
            });
            data.push(todo);
        }
    }

    (path_buf.into_os_string().into_string().unwrap(), data)
}

/// Add items to the todo list.
fn add_items(data: &mut Vec<Todo>, params: Vec<String>, data_path: &String) {
    for param in params {
        data.push(Todo {
            label: param,
            complete: false,
        });
    }

    write_data(data, data_path);
}

/// Remove items from the todo list.
/// Items are specified by their position (as shown in "todo list" command) or with "all".
fn remove_items(data: &mut Vec<Todo>, params: Vec<String>, data_path: &String) {
    if params.len() == 0 {
        eprintln!("ERROR: Invalid use of `remove`. See `todo help` for options");
        process::exit(1);
    }
    if params[0] == "all" {
        data.clear();
        write_data(data, data_path);
        return;
    } else if params[0] == "checked" || params[0] == "completed" {
        data.retain(|item| item.complete == false);
        write_data(data, data_path);
        return;
    }

    let mut positions: Vec<usize> = params
        .iter()
        .map(|s| s.parse::<usize>().unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot convert position string \"{s}\" into a valid position value: {err}");
            process::exit(1);
        })).collect();

    positions.sort();
    positions.reverse();

    // Out-of-bound positions are ignored
    for pos in positions {
        if pos <= data.len() {
            data.remove(pos - 1);
        }
    }

    write_data(data, data_path);
}

/// Check items in the todo list.
fn check_items(data: &mut Vec<Todo>, params: Vec<String>, data_path: &String) {
    if params.len() == 0 {
        eprintln!("ERROR: Invalid use of `check`. See `todo help` for options");
        process::exit(1);
    }
    if params[0] == "all" {
        for item in data.iter_mut() {
            item.complete = true;
        }
        write_data(data, data_path);
        return;
    }

    let positions: Vec<usize> = params
        .iter()
        .map(|s| s.parse::<usize>().unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot convert position string \"{s}\" into a valid position value: {err}");
            process::exit(1);
        })).collect();

    // Out-of-bound positions are ignored
    for pos in positions {
        if pos <= data.len() {
            data[pos - 1].complete = true;
        }
    }

    write_data(data, data_path);
}

/// Uncheck items in the todo list.
fn uncheck_items(data: &mut Vec<Todo>, params: Vec<String>, data_path: &String) {
    if params.len() == 0 {
        eprintln!("ERROR: Invalid use of `uncheck`. See `todo help` for options");
        process::exit(1);
    }
    if params[0] == "all" {
        for item in data.iter_mut() {
            item.complete = false;
        }
        write_data(data, data_path);
        return;
    }

    let positions: Vec<usize> = params
        .iter()
        .map(|s| s.parse::<usize>().unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot convert position string \"{s}\" into a valid position value: {err}");
            process::exit(1);
        })).collect();

    // Out-of-bound positions are ignored
    for pos in positions {
        if pos <= data.len() {
            data[pos - 1].complete = false;
        }
    }

    write_data(data, data_path);
}

/// Sort items (by default the completed items will be listed last).
/// TODO: implement param options for sorting (i.e., completed first or completed last)
fn sort_items(data: &mut Vec<Todo>, _params: Vec<String>, data_path: &String) {
    data.sort_by_key(|item| item.complete);
    write_data(data, data_path);
}

/// Print the todo list
fn print_list(data: &Vec<Todo>) {
    if data.len() == 0 {
        println!("Nothing to do!\n\nRun `todo help` for help.");
        return;
    }

    for (i, item) in data.iter().enumerate() {
        println!(
            "{}",
            if item.complete {
                format!("☑ {}: {}", i + 1, item.label).green()
            } else {
                format!("☐ {}: {}", i + 1, item.label).white()
            }
        );
    }
}

/// Write todo data to disk
fn write_data(data: &Vec<Todo>, data_path: &String) {
    let mut buf = String::new();
    for item in data {
        let item_serialized = serde_json::to_string(item).unwrap_or_else(|err| {
            eprintln!("ERROR: Could not serialize the todo item into JSON format: {err}");
            process::exit(1);
        });
        buf.push_str(&item_serialized);
        buf.push('\n');
    }

    fs::write(data_path, buf).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not write to the data file: {err}");
        process::exit(1);
    });
}

/// Print the help information
fn show_help() {
    println!("
add <items...>
        Add item(s) to the todo list

edit <item_positions...>
        Edit item(s) in the todo list

list
        Print the todo list. Use the numeric positions listed for commands with <item_positions...> parameters

remove <item_positions...> | \"all\" | \"checked\" | \"completed\"
        Remove item(s) from the todo list

clear
        Clears all items from the todo list (equivalent to \"remove all\")

check <item_positions...> | \"all\" 
        Mark item(s) as completed

uncheck <item_positions...> | \"all\" 
        Mark item(s) as incomplete

sort 
        Sort items such that completed items appear last

set(?) <setting> <option>
        Change config setting to have value <option>

Any parameters with <...> signify that you can use multiple space-separated parameters.
Any action marked with a (?) has further documentation (i.e, run `todo set help`)");
}

/// Extract settings from config file.
/// If a config doesn't exist, make one.
fn extract_settings() -> Settings {
    let mut config_path = dirs::config_dir().unwrap_or_else(|| {
        eprintln!("ERROR: Could not find config directory.");
        process::exit(1);
    });

    config_path.push("todo-app");

    fs::create_dir_all(&config_path).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not create config file: {err}");
        process::exit(1);
    });

    config_path.push("settings.json");

    if config_path.exists() {
        let settings_str = fs::read_to_string(config_path).unwrap();
        let settings: Settings = serde_json::from_str(&settings_str).unwrap_or_else(|err| {
            eprintln!("ERROR: Could not parse settings file: {err}");
            process::exit(1);
        });
        return settings;
    }

    let settings = Settings {
        silent: String::from("off"),
    };
    write_settings(&config_path, &settings);
    settings
}

fn set_setting(settings: &mut Settings, params: Vec<String>) {
    let setting_choices = vec![(
        "silent",
        vec![String::from("on"), String::from("off")],
        "Don't print the todo list after each mutation command (Default = off)",
    )];

    if params.len() >= 1 && params[0] == "help" {
        print_setting_help(setting_choices);
        return;
    }

    let mut setting_map = HashMap::from([("silent", &mut settings.silent)]);

    if params.len() != 2 {
        eprintln!(
            "ERROR: Parameter format is incorrect. See `todo set help` for information.\nUsage: todo set <setting> <value>"
        );
        process::exit(1);
    }

    let mut success = false;

    for opt in setting_choices {
        if opt.0 == params[0] {
            if opt.1.contains(&params[1]) {
                let setting = setting_map.get_mut(opt.0).unwrap();
                setting.clear();
                setting.push_str(&params[1]);
                success = true;
            }
        }
    }

    if !success {
        eprintln!(
            "ERROR: Failed to change setting \"{}\" to option \"{}\", setting or option doesn't exist.",
            params[0], params[1]
        );
        process::exit(1);
    }

    let mut settings_path = dirs::config_dir().unwrap();
    settings_path.push("todo-app/settings.json");
    write_settings(&settings_path, settings);

    println!(
        "Successfully changed setting \"{}\" to \"{}\".",
        params[0], params[1]
    );
}

/// Show help for settings
fn print_setting_help(setting_choices: Vec<(&'static str, Vec<String>, &'static str)>) {
    println!(
        "Change settings with \"todo set <setting> <option>\".
Commands:"
    );
    for setting in setting_choices {
        print!("\t{} <", setting.0);
        for (i, opt) in setting.1.iter().enumerate() {
            print!(
                "{}{}",
                opt,
                if i < setting.1.len() - 1 {
                    " | ".to_string()
                } else {
                    format!(">\t{}\n", setting.2)
                }
            );
        }
    }
}

/// Edit an item
fn edit_item(data: &mut Vec<Todo>, params: Vec<String>, data_path: &String) {
    if params.len() == 0 {
        eprintln!("ERROR: Invalid use of `edit`. See `todo help` for options");
        process::exit(1);
    }

    let positions: Vec<usize> = params
        .iter()
        .map(|s| s.parse::<usize>().unwrap_or_else(|err| {
            eprintln!("ERROR: Cannot convert position string \"{s}\" into a valid position value: {err}");
            process::exit(1);
        })).collect();

    for pos in positions {
        if pos <= data.len() {
            let original = &data[pos - 1];
            println!("Original: {}", original.label);

            print!("New: ");
            io::stdout().flush().expect("Failed to flush stdout");

            let mut buffer = String::new();
            let stdin = io::stdin();
            stdin.read_line(&mut buffer).unwrap_or_else(|err| {
                eprintln!("ERROR: Could not read user input: {err}");
                process::exit(1);
            });

            data[pos - 1].label = buffer.trim_end().to_string();
        }
    }

    write_data(data, data_path);
}

/// Write settings to disk.
fn write_settings(path: &PathBuf, settings: &Settings) {
    let settings_str = serde_json::to_string(&settings).unwrap();
    fs::write(path, settings_str).unwrap_or_else(|err| {
        eprintln!("ERROR: Could not create the config file: {err}");
        process::exit(1);
    });
}
