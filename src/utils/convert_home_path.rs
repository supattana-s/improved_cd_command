use std::path::PathBuf;

pub fn convert_home_path(path: &PathBuf) -> String {
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
