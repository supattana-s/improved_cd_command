use std::fs;
use std::path::PathBuf;

pub fn list_command(current_dir: &PathBuf) {
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
