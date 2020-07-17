use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    let mut state = argus_core::state::State::init();

    loop {
        print!("{}[2J", 27 as char);
        println!("Welcome to Argus");
        println!("----> 1 - Create New User");
        println!("----> 2 - View Users");
        println!("----> 3 - Login with ID and PIN");
        println!("----> 4 - Quit");

        let mut x = String::new();
        print!("----> ",);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut x).expect("Failed to read line");

        match x.trim().parse::<i32>().unwrap() {
            1 => {
                // println!("create new user");
                // let mut new_user_name = String::new();
                //
                // println!("[ argus ] Please enter your name ----> ",);
                // io::stdout().flush().unwrap();
                // io::stdin()
                //     .read_line(&mut new_user_name)
                //     .expect("Failed to read line");

                let new_account = argus_core::account::Account::generate();
                let new_account_information = new_account.information();

                state.save_account_information(&new_account, new_account_information);

                thread::sleep_ms(10000);
            }
            2 => {
                println!("view users");

                for k in state.account_salt_and_sha.keys() {
                    println!("{:?}", k);
                }

                // for value in system.public_account_information.values() {
                //     let sha = value.get(&String::from("sha"));
                //     println!("{:?}", sha);
                //     let x25519_public_key = value.get(&String::from("x25519_public_key"));
                //     println!("{:?}", x25519_public_key);
                //     let ed25519_public_key = value.get(&String::from("ed25519_public_key"));
                //     println!("{:?}", ed25519_public_key);
                // }
                //
                // for k in system.public_account_information.keys() {
                //     println!("{}", k);
                // }
                //
                // for value in system.public_account_information.values() {
                //     let sha = value.get(&String::from("sha"));
                //     println!("{:?}", sha);
                //     let x25519_public_key = value.get(&String::from("x25519_public_key"));
                //     println!("{:?}", x25519_public_key);
                //     let ed25519_public_key = value.get(&String::from("ed25519_public_key"));
                //     println!("{:?}", ed25519_public_key);
                // }

                thread::sleep_ms(10000);
            }
            3 => {
                print!("{}[2J", 27 as char);
                println!("login with id and pin");

                let mut account_to_use = String::new();

                println!("[ argus ] Please enter account id ----> ",);
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut account_to_use)
                    .expect("Failed to read line");

                let len = &account_to_use.len();
                account_to_use.truncate(len - 1);

                let mut user = state.load_account_information(&account_to_use);
                let user_session = argus_core::session::Session::init();
                let system = argus_core::session::Session::init();
                let user_x25519_public_key = user_session.x25519_public_key();
                let system_x25519_public_key = system.x25519_public_key();
                let user_shared_secret =
                    user_session.x25519_shared_secret(&system_x25519_public_key);
                let system_shared_secret = system.x25519_shared_secret(&user_x25519_public_key);

                if user_shared_secret.as_bytes() == system_shared_secret.as_bytes() {
                    println!("secure connection established");
                } else {
                    println!("please try again...",);
                    thread::sleep_ms(10000);
                    continue;
                }

                loop {
                    println!("Welcome");

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
            4 => {
                println!("----> Goodbye ...");
                print!("{}[2J", 27 as char);
                return;
            }
            _ => continue,
        };
    }
}
