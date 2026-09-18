use std::fs;
use std::io;

fn main() {
    let mut path = String::new();

    if io::stdin().read_line(&mut path).is_err() {
        println!("failure");
        return;
    }

    let trimmed_path = path.trim();

    if trimmed_path.is_empty() {
        println!("failure");
        return;
    }

    match fs::read(trimmed_path) {
        Ok(_) => println!("success"),
        Err(_) => println!("failure"),
    }
}
