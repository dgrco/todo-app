use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::{fs, process};

const DATA_FILE_NAME: &'static str = "todo.dat";

#[derive(Serialize, Deserialize)]
struct Todo {
    label: String,
    complete: bool,
}

/// Run the todo app.
/// @param action - The action string chosen by the user.
/// @param params - Any parameters passed after the action.
pub fn run(action: &String, params: Vec<String>) {
    let (data_path, mut todo_data) = read_to_vec(dirs::data_dir());
    match action.as_str() {
        "add" => add_items(&mut todo_data, params, &data_path),
        "list" => print_list(&todo_data),
        "remove" => remove_items(&mut todo_data, params, &data_path),
        "clear" => remove_items(&mut todo_data, vec!["all".to_string()], &data_path),
        "check" => check_items(&mut todo_data, params, &data_path),
        "uncheck" => uncheck_items(&mut todo_data, params, &data_path),
        "sort" => sort_items(&mut todo_data, params, &data_path),
        _ => {}
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
    for (i, item) in data.iter().enumerate() {
        println!(
            "{}",
            if item.complete {
                format!(
                    "☑ {}: {}",
                    i + 1,
                    item.label
                ).green()
            } else {
                format!(
                    "☐ {}: {}",
                    i + 1,
                    item.label
                ).white()
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
