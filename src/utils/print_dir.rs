use std::io::{stdout, Write};

pub fn print_one_line(print_item: &String) {
    print!("{} ", print_item);
    stdout().flush().unwrap();
}
