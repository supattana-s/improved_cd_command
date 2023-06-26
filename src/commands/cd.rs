use home::home_dir;
use std::env;
use std::path::{Path, PathBuf};

pub fn cd_command(input: String, current_dir: &PathBuf) -> PathBuf {
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
