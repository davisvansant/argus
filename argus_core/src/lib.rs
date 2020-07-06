use std::collections::HashMap;

pub enum X25519 {
    EphemeralSecret(argus_x25519::EphemeralSecret),
    PublicKey(argus_x25519::PublicKey),
}

pub struct User {
    pub account_number: String,
    pub pin: String,
}

impl Default for User {
    fn default() -> Self {
        User::new()
    }
}

impl User {
    pub fn new() -> Self {
        Self {
            account_number: argus_rand::generate_account_number(),
            pin: argus_rand::generate_pin(),
        }
    }

    pub fn pin(&self) -> HashMap<&str, String> {
        let mut user_pin: HashMap<&str, String> = HashMap::new();
        let salt = argus_rand::generate_salt();
        let sha = argus_sha2::digest_512::build_object(&self.pin, &salt);
        user_pin.insert(&self.account_number, sha);
        user_pin.insert(&self.pin, salt);
        user_pin
    }

    pub fn x25519_keys(&self) -> HashMap<&str, X25519> {
        let mut x25519_pair: HashMap<&str, X25519> = HashMap::new();
        let x25519_secret = argus_x25519::generate_ephermeral_secret();
        let x25519_public = argus_x25519::generate_public_key(&x25519_secret);
        x25519_pair.insert(&self.account_number, X25519::EphemeralSecret(x25519_secret));
        x25519_pair.insert(&self.account_number, X25519::PublicKey(x25519_public));
        x25519_pair
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_user() {
        // let test_user_name = String::from("user - tester");
        // let test_account_number = String::from("account - 12345");
        // let test_account_pin = String::from("pin - 12345");
        // let test_salt = String::from("salt - 12345");
        // let test_hash = String::from("hash - 12345");
        let test_user: User = User::new();
        // let test_user_x25519 = User::x25519_public_key();
        let test_user_keys = test_user.pin();
        let test_user_keys = test_user.x25519_keys();
        // let some_awesome_hash = User::link(
        //     &test_account_number,
        //     &test_hash,
        //     &test_account_pin,
        //     &test_salt,
        // );
        // assert!(test_user_name.contains("user - tester"));
        // assert!(test_account_number.contains("account - 12345"));
        // assert!(test_account_pin.contains("pin - 12345"));
        // assert!(test_salt.contains("salt - 12345"));
        // assert!(test_hash.contains("hash - 12345"));
        // assert!(test_user.name.contains("user - tester"));
        // assert!(test_user.account_number.contains("account - 12345"));
        // assert!(test_user.pin.contains("pin - 12345"));
        // assert!(some_awesome_hash.contains_key(&test_account_number));
        // assert!(some_awesome_hash.contains_key(&test_account_pin));
    }
}
