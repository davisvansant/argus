use std::io;
use std::io::Write;

fn main() {
    let system = argus_core::system::System::init();
    let system_x25519_public_key = system.x25519_public_key();

    loop {
        print!("{}[2J", 27 as char);
        println!("Welcome to Argus");
        println!("----> 1 - Create New User");
        println!("----> 2 - Login with ID and PIN");
        println!("----> 3 - Quit");

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
            }
            2 => {
                print!("{}[2J", 27 as char);
                println!("login with id and pin");
                loop {
                    println!("Welcome username");
                    println!("[ argus ] Secure connection verified ....");
                    println!("----> 1 - Create New Secret");
                    println!("----> 2 - View current secrets");
                    println!("----> 3 - Logout");

                    let mut x = String::new();
                    print!("----> ",);
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut x).expect("Failed to read line");

                    match x.trim().parse::<i32>().unwrap() {
                        1 => {
                            println!("create new secrets",);
                        }
                        2 => {
                            println!("view current secrets",);
                        }
                        3 => {
                            println!("----> Goodbye ...");
                            print!("{}[2J", 27 as char);
                            break;
                        }
                        _ => continue,
                    }
                }
            }
            3 => {
                println!("----> Goodbye ...");
                print!("{}[2J", 27 as char);
                return;
            }
            _ => continue,
        };
    }
}
