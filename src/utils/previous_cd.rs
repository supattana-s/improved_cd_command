use std::{collections::HashMap, path::PathBuf};

use dialoguer::Select;

pub fn previous_cd<'a>(
    visited_path: &'a HashMap<String, Vec<PathBuf>>,
    input: &String,
) -> &'a PathBuf {
    if visited_path.get(input).unwrap().len() == 1 {
        &visited_path.get(input).unwrap()[0]
    } else {
        let mut list_for_selection: Vec<String> = vec![];
        let all_visited_path = visited_path.get(input).unwrap();
        for item in all_visited_path.into_iter() {
            list_for_selection.push(item.clone().into_os_string().into_string().unwrap());
        }
        let selected_path_index = Select::new().items(&list_for_selection).interact();
        &visited_path.get(input).unwrap()[selected_path_index.unwrap()]
    }
}
