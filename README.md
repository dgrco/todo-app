# Todo App (Made With Rust)

A Todo App that is simple, yet is actually useful.

## Features
- Adding/Removing Todos\\
`todo add "first todo" "second todo" todo3 todo4` \\
`todo remove 2 3` (remove specific todos),  `todo remove all`, `todo remove checked` (or `todo remove completed`)

- Listing Todos\\
`todo list` - All todos have a checkbox. Completed todos will have this checkbox checked, and will be colored green.\\
(This command is useful for identifying the positions of todos that is used in position-specific commands.)

- Checking/Unchecking Todos\\
`todo check 1 2` (checking specific todos), `todo check all`\\
`todo uncheck 1 2` (unchecking specific todos), `todo uncheck all`

- Sorting Todos\\
`todo sort` - Sorts todos such that the completed todos will be positioned last.

## Install
To install this program, you should have Cargo installed (via rustup or by other means).\\

This program will not (yet) be uploaded to crates.io, so for now it must be built from source using Cargo. Please do the following:\\
- Clone this repository using git: `git clone git@github.com:dgrco/todo-app.git` (using SSH), or `git clone https://github.com/dgrco/todo-app.git` (using HTTPS).
- Change your working directory to the newly downloaded folder: `cd todo-app`.
- Build the project: `cargo build --release`.
- Install it locally: `cargo install --path <path>`. You can install it in Cargo's binary folder using `cargo install --path .`.
- Make sure the path entered above is in your PATH.
- Run the program using `todo`.
