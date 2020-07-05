use std::collections::HashMap;

struct User {
    name: String,
    account_number: String,
    pin: String,
}

impl User {
    fn new(name: &str, account_number: &str, pin: &str) -> User {
        User {
            // this can take user input
            name: name.to_string(),
            // argus_rand::generate_account_number();
            account_number: account_number.to_string(),
            // argus_rand::generate_pin();
            pin: pin.to_string(),
        }
    }

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
            &(*test_account_number).to_string(),
            &(*test_account_pin).to_string(),
        );

        let some_awesome_hash = User::link(
            &test_account_number,
            &test_hash,
            &test_account_pin,
            &test_salt,
        );
        assert!(test_user_name.contains("user - tester"));
        assert!(test_account_number.contains("account - 12345"));
        assert!(test_account_pin.contains("pin - 12345"));
        assert!(test_salt.contains("salt - 12345"));
        assert!(test_hash.contains("hash - 12345"));
        assert!(test_user.name.contains("user - tester"));
        assert!(test_user.account_number.contains("account - 12345"));
        assert!(test_user.pin.contains("pin - 12345"));
        assert!(some_awesome_hash.contains_key(&test_account_number));
        assert!(some_awesome_hash.contains_key(&test_account_pin));
    }
}
