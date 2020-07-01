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

        let mut new_user_name = String::new();

        println!("[ argus ] Please enter your name ----> ",);
        io::stdout().flush();
        io::stdin()
            .read_line(&mut new_user_name)
            .expect("Failed to read line");

        loop {
            println!("[ argus ] Please enter your password ----> ");

            let mut new_user_password = String::new();

            io::stdout().flush();
            io::stdin()
                .read_line(&mut new_user_password)
                .expect("Failed to read line");

            let mut new_user_password_again = String::new();
            io::stdout().flush();

            println!("[ argus ] Please enter your password ----> ");
            io::stdout().flush();
            io::stdin()
                .read_line(&mut new_user_password_again)
                .expect("Failed to read line");

            if new_user_password == new_user_password_again {
                println!("good!");
                break;
            } else {
                println!("not good!");
                continue;
            }
        }

        println!("[ argus ] creating key...");

        println!("[ argus ] creating another key....");

        println!("----> 1 - Create new item");
        println!("----> 2 - View Current items");
        println!("----> 3 - Add another user for testing");
        println!("----> 4 - Quit");

        let mut x = String::new();
        print!("----> ",);
        io::stdout().flush();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<i32>().unwrap() {
            1 => println!("create new user"),
            2 => println!("view current items!"),
            3 => println!("add another user!"),
            4 => {
                println!("----> Goodbye ...");
                print!("{}[2J", 27 as char);
                return;
            }
            _ => continue,
        };
    }
}
