use std::io;
use std::io::Write;

fn main() {
    loop {
        println!("Welcome to Argus");

        let mut new_user_name = String::new();

        println!("[ argus ] Please enter your name ----> ",);
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut new_user_name)
            .expect("Failed to read line");

        println!("[ argus ] creating PIN...");

        println!("[ argus ] creating Key....");

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
