use std::collections::HashMap;
use std::io;
use std::io::Write;

struct User {
    name: String,
    id: String,
}

impl User {
    fn new(name: &str, id: &str) -> User {
        User {
            name: name.to_string(),
            id: id.to_string(),
        }
    }
}

fn main() {
    loop {
        println!("Welcome to Argus");

        let mut x = String::new();
        print!("----> ",);
        io::stdout().flush();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<i32>().unwrap() {
            1 => println!("create new user"),
            _ => return,
        };
    }
}
