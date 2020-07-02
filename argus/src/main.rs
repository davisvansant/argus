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
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut new_user_name)
            .expect("Failed to read line");

        // loop {
        //     println!("[ argus ] Please enter your new PIN ----> ");
        //
        //     let mut new_user_pin = String::new();
        //
        //     io::stdout().flush().unwrap();
        //     io::stdin()
        //         .read_line(&mut new_user_pin)
        //         .expect("Failed to read line");
        //
        //     let mut confirm_new_user_pin = String::new();
        //     io::stdout().flush().unwrap();
        //
        //     println!("[ argus ] Please confirm your new PIN ----> ");
        //     io::stdout().flush().unwrap();
        //     io::stdin()
        //         .read_line(&mut confirm_new_user_pin)
        //         .expect("Failed to read line");
        //
        //     if new_user_pin == confirm_new_user_pin {
        //         println!("good!");
        //         break;
        //     } else {
        //         println!("not good!");
        //         continue;
        //     }
        // }

        println!("[ argus ] creating PIN...");

        println!("[ argus ] creating Key....");

        println!("----> 1 - Create New Secret");
        println!("----> 2 - View Current Secrets");
        println!("----> 3 - Add another user for testing");
        println!("----> 4 - Generate New PIN");
        println!("----> 5 - Quit");

        let mut x = String::new();
        print!("----> ",);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<i32>().unwrap() {
            1 => println!("create new user"),
            2 => println!("view current items!"),
            3 => println!("add another user!"),
            4 => println!("generate new pin"),
            5 => {
                println!("----> Goodbye ...");
                print!("{}[2J", 27 as char);
                return;
            }
            _ => continue,
        };
    }
}
