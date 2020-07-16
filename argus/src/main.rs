use std::io;
use std::io::Write;
use std::{thread, time};

fn main() {
    let mut system = argus_core::system::System::init();

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
                println!("create new user");
                let mut new_user_name = String::new();

                println!("[ argus ] Please enter your name ----> ",);
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut new_user_name)
                    .expect("Failed to read line");

                let new_user_private_information = argus_core::user::User::new();
                let new_user_public_information =
                    argus_core::user::User::generate(&new_user_private_information);

                system.save_public_account_information(
                    &new_user_private_information,
                    new_user_public_information,
                );
                system.save_private_account_information(new_user_private_information);

                thread::sleep_ms(10000);
            }
            2 => {
                println!("view users");

                for k in system.public_account_information.keys() {
                    println!("{}", k);
                }

                for value in system.public_account_information.values() {
                    let sha = value.get(&String::from("sha"));
                    println!("{:?}", sha);
                    let x25519_public_key = value.get(&String::from("x25519_public_key"));
                    println!("{:?}", x25519_public_key);
                    let ed25519_public_key = value.get(&String::from("ed25519_public_key"));
                    println!("{:?}", ed25519_public_key);
                }

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

                println!("{}", &account_to_use);
                println!("{:?}", &account_to_use.len());
                assert!(&account_to_use.contains(&account_to_use));

                let system_x25519_public_key = system.x25519_public_key();

                let current_user_private_information =
                    system.load_private_account_information(&account_to_use);
                // let current_user_public_information =
                //     system.load_public_account_information(&account_to_use);
                // println!("{:?}", current_user_private_information.account_number);

                // for key in system.public_account_information.keys() {
                //     if key.contains(&account_to_use) {
                //         println!("yes!");
                //         let load_user = system.private_account_information.get(&account_to_use).unwrap();
                //         println!("ready to load - {:?}", load_user.pin);
                //     } else {
                //         println!("no", );
                //     }
                // }

                let x25519_public_key = current_user_private_information.public_key();
                println!("{:?}", x25519_public_key);
                let ed25519_public_key = current_user_private_information.ed25519_public_key();
                println!("{:?}", ed25519_public_key);
                let current_user_shared_secret = current_user_private_information
                    .x25519_shared_secret(&system_x25519_public_key);
                println!("{:?}", current_user_shared_secret.as_bytes());
                let system_shared_secret = system.x25519_shared_secret(&x25519_public_key);
                println!("{:?}", &system_shared_secret.as_bytes());

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
