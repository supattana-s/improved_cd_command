use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::{env, fs};

use home::home_dir;

fn exit_command() {
    println!("Bye!!!");
    exit(0);
}

fn list_command(current_dir: &PathBuf) {
    let all_items = fs::read_dir(&current_dir).unwrap();
    for item in all_items {
        let element = item.unwrap();
        if element.metadata().unwrap().is_dir() {
            println!("directory \t {:?}", element.file_name());
        } else if element.metadata().unwrap().is_file() {
            println!("File \t\t {:?}", element.file_name());
        } else {
            println!("unknown \t\t {:?}", element.file_name());
        }
    }
}

fn cd_command(input: String, current_dir: &PathBuf) -> PathBuf {
    let vec_input: Vec<&str> = input.split(" ").collect();
    // let current_dir = env::current_dir().unwrap();
    let home_dir = home_dir().unwrap();

    if vec_input.len() == 1 {
        env::set_current_dir(&home_dir).unwrap();
        home_dir
    } else {
        let changing_dir: Vec<&str> = vec_input[1].split("/").collect();

        // println!("{:?}", changing_dir);

        if changing_dir[0] == ".." && changing_dir.len() == 1 {
            let parent_path = current_dir.parent().unwrap();
            env::set_current_dir(parent_path).unwrap();
            PathBuf::from(parent_path)
        } else if changing_dir[0] == "." {
            let mut full_target_path_string = String::new();
            full_target_path_string.push_str(&current_dir.to_str().unwrap());

            for (idx, item) in changing_dir.iter().enumerate() {
                if idx == 0 {
                    continue;
                }
                full_target_path_string.push_str("/");
                full_target_path_string.push_str(item);
            }

            let full_target_path = Path::new(&full_target_path_string);
            if full_target_path.is_dir() {
                env::set_current_dir(full_target_path).unwrap();
                PathBuf::from(full_target_path)
            } else {
                println!("{} is not such a directory!!!", full_target_path.display());
                PathBuf::from(current_dir)
            }
        } else {
            PathBuf::from(current_dir)
        }
    }
}

fn convert_home_path(path: &PathBuf) -> String {
    let home_path: Vec<&str> = path.to_str().unwrap().split("/").collect();
    if home_path.len() <= 3 {
        path.to_str().unwrap();
    }
    let mut new_home_path = String::new();
    new_home_path.push_str("~");
    for (i, path) in home_path.iter().enumerate() {
        if i == 0 || i == 1 || i == 2 {
            continue;
        }
        new_home_path.push_str("/");
        new_home_path.push_str(path);
    }
    new_home_path
}

fn print_one_line(print_item: &String) {
    print!("{} ", print_item);
    stdout().flush().unwrap();
}

fn store_visited_path(path: &PathBuf, store_hash_map: &mut HashMap<String, PathBuf>) {
    let storing_path = path.clone();
    let path_str = String::from(path.to_str().unwrap());
    let seperated_path_collection: Vec<&str> = path_str.split("/").collect();
    let key_string = String::from(seperated_path_collection[seperated_path_collection.len() - 1]);
    store_hash_map.insert(key_string, storing_path);
}

fn previous_cd<'a>(
    visited_path: &'a HashMap<String, PathBuf>,
    input: &String,
    current_dir: &'a PathBuf,
) -> &'a PathBuf {
    if visited_path.is_empty() {
        println!("Invalid Command");
        current_dir
    } else {
        visited_path.get(input).unwrap()
    }
}

fn main() {
    //let mut visited_path: Vec<String> = Vec::new();
    let mut visited_path: HashMap<String, PathBuf> = HashMap::new();
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
                } else {
                    current_dir = previous_cd(&visited_path, &input, &current_dir).clone();
                }
                print_one_line(&convert_home_path(&current_dir));
            }
            Err(err) => println!("Error occur: {}", err),
        }
    }
}
