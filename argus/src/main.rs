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

        // create pin here from argus_rand::generate_pin();
        // store pin, salt, hash for reuse

        println!("[ argus ] creating Key....");

        // create x25519 public/private key here
        // let user_private_key = argus_x25519::generate_keypair();

        // let user_secret = argus_x25519::generate_ephermeral_secret();
        // let argus_secret = argus_x25519::generate_ephermeral_secret();
        // let user_public = argus_x25519::generate_public_key(&user_secret);
        // let argus_public = argus_x25519::generate_public_key(&argus_secret);
        // let user_shared_secret = user_secret.diffie_hellman(&argus_public);
        // let argus_shared_secret = argus_secret.diffie_hellman(&user_public);

        // println!("[ argus ] Secure connection verified ....");

        println!("----> 1 - Create New Secret");

        // first verify with pin (cant create without correct pin)
        // create some message - let message: &[u8] = b"This is a test of the tsunami alert system.";
        // generate keypair for secret - let keypair = argus_ed25519::generate_keypair();
        // store keypair for secret
        // sign secret message - let signature = argus_ed25519::generate_signature(&keypair, &message);
        // store secret with signature
        // create public key for later use
        // let public_key = argus_ed25519::generate_public_key(&keypair);

        println!("----> 2 - View Current Secrets");

        // first verify with pin (cant create without correct pin)
        // view specified secret with key -

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
