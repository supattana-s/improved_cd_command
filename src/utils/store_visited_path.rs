use std::collections::HashMap;
use std::path::PathBuf;

pub fn store_visited_path(path: &PathBuf, store_hash_map: &mut HashMap<String, Vec<PathBuf>>) {
    let mut storing_path = vec![path.clone()];
    let path_str = String::from(path.to_str().unwrap());
    let seperated_path_collection: Vec<&str> = path_str.split("/").collect();
    let key_string = String::from(seperated_path_collection[seperated_path_collection.len() - 1]);

    if store_hash_map.contains_key(&key_string) {
        let mut new_storing_vector: Vec<PathBuf> = store_hash_map.get(&key_string).unwrap().clone();
        println!("{:?}", new_storing_vector);
        if !store_hash_map.get(&key_string).unwrap().contains(&path) {
            println!("yes");
            new_storing_vector.append(&mut storing_path);
            store_hash_map.insert(key_string, new_storing_vector);
        } else {
            println!("Hi")
        }
    } else {
        store_hash_map.insert(key_string, storing_path);
    }
}
