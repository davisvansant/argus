// use argus_x25519;
use std::collections::HashMap;

struct User {
    name: String,
    account_number: String,
    pin: String,
    x25519_secret: argus_x25519::EphemeralSecret,
    // x25519_public_key: argus_x25519::PublicKey,
}

impl User {
    // fn new(name: &str, account_number: &str, pin: &str) -> User {
    fn new(name: &str) -> Self {
        Self {
            // this can take user input
            // name: self.name.to_string(),
            name: name.to_string(),
            account_number: argus_rand::generate_account_number(),
            // account_number: self.account_number.to_string(),
            pin: argus_rand::generate_pin(),
            // pin: self.pin.to_string(),
            // x25519_secret: self.x25519_secret.to_string(),
            // x25519_private_key: self.x25519_private_key.to_string(),
            // x25519_public_key: self.x25519_public_key.to_string(),
            x25519_secret: argus_x25519::generate_ephermeral_secret(),
            // x25519_public_key: argus_x25519::generate_public_key(&Self { x25519_secret }),
        }
    }

    // fn x25519_public_key(&self) -> HashMap<String, argus_x25519::PublicKey> {
    //     let mut x25519: HashMap<String, argus_x25519::PublicKey> = HashMap::new();
    //     // let x25519_secret = generate_ephermeral_secret();
    //     let x25519_public = argus_x25519::generate_public_key(&self.x25519_secret);
    //     // x25519_secret.insert(id.account_number, x25519_secret);
    //     x25519.insert(self.account_number, x25519_public);
    //     x25519
    // }

    fn link(
        account_number: &str,
        hash: &str,
        account_pin: &str,
        salt: &str,
    ) -> HashMap<String, String> {
        let mut link: HashMap<String, String> = HashMap::new();
        link.insert((&account_number).to_string(), (&hash).to_string());
        link.insert((&account_pin).to_string(), (&salt).to_string());
        link
    }
}

// struct X25519 {
//     secret: String,
//     private_key: String,
//     public_key: String,
// }
//
// impl X25519 {
//     fn new(id: &User) -> HashMap<String, String> {
//         let mut x25519: HashMap<String, String> = HashMap::new();
//         let x25519_secret = generate_ephermeral_secret();
//         let x25519_public = generate_public_key(&x25519_secret);
//         x25519_secret.insert(id.account_number, x25519_secret);
//         x25519_public.insert(id.account_number, x25519_public);
//         x25519
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_user() {
        let test_user_name = String::from("user - tester");
        let test_account_number = String::from("account - 12345");
        let test_account_pin = String::from("pin - 12345");
        let test_salt = String::from("salt - 12345");
        let test_hash = String::from("hash - 12345");

        let test_user: User = User::new(
            &test_user_name,
            // &(*test_account_number).to_string(),
            // &(*test_account_pin).to_string(),
        );

        let some_awesome_hash = User::link(
            &test_account_number,
            &test_hash,
            &test_account_pin,
            &test_salt,
        );
        assert!(test_user_name.contains("user - tester"));
        // assert!(test_account_number.contains("account - 12345"));
        // assert!(test_account_pin.contains("pin - 12345"));
        // assert!(test_salt.contains("salt - 12345"));
        // assert!(test_hash.contains("hash - 12345"));
        // assert!(test_user.name.contains("user - tester"));
        // assert!(test_user.account_number.contains("account - 12345"));
        // assert!(test_user.pin.contains("pin - 12345"));
        assert!(some_awesome_hash.contains_key(&test_account_number));
        assert!(some_awesome_hash.contains_key(&test_account_pin));
    }
}
