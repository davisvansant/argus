use std::io;
use std::io::Write;

fn main() {
    let system = argus_core::system::System::init();
    let system_x25519_public_key = system.x25519_public_key();

    loop {
        println!("Welcome to Argus");
        println!("----> 1 - Create New User");
        println!("----> 2 - View Current Secrets");
        println!("----> 3 - Add another user for testing");
        println!("----> 4 - Generate New PIN");
        println!("----> 5 - Quit");

        let mut x = String::new();
        print!("----> ",);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<i32>().unwrap() {
            1 => {
                println!("create new user");
                let mut new_user_name = String::new();

                println!("[ argus ] Please enter your name ----> ",);
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut new_user_name)
                    .expect("Failed to read line");

                println!("[ argus ] creating PIN...");

                println!("[ argus ] creating Key....");

                // println!("[ argus ] Secure connection verified ....");
            },
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
