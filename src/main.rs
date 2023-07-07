use dialoguer::Select;
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::path::PathBuf;

mod commands;
use commands::cd::cd_command;
use commands::exit::exit_command;
use commands::ls::list_command;

mod utils;
use utils::convert_home_path::convert_home_path;
use utils::previous_cd::previous_cd;
use utils::print_dir::print_one_line;
use utils::store_visited_path::store_visited_path;

fn main() {
    //let mut visited_path: Vec<String> = Vec::new();
    let mut visited_path: HashMap<String, Vec<PathBuf>> = HashMap::new();
    let mut current_dir = env::current_dir().unwrap();
    let current_path = convert_home_path(&current_dir);
    let stdin = stdin();
    print_one_line(&current_path);

    for line in stdin.lines() {
        match line {
            Ok(input) => {
                let splitted_input: Vec<&str> = input.split(" ").collect();
                if splitted_input[0] == "exit" {
                    exit_command();
                } else if splitted_input[0] == "ls" {
                    list_command(&current_dir);
                } else if splitted_input[0] == "cd" {
                    current_dir = cd_command(input, &current_dir);
                    store_visited_path(&current_dir, &mut visited_path);
                    println!("{:?}", visited_path);
                } else if input == "" {
                    print_one_line(&convert_home_path(&current_dir));
                    continue;
                } else {
                    if visited_path.is_empty() {
                        println!("Invalid Command");
                    } else {
                        current_dir = previous_cd(&visited_path, &input).clone();
                    }
                }
                print_one_line(&convert_home_path(&current_dir));
            }
            Err(err) => println!("Error occur: {}", err),
        }
    }
}
