pub struct User {
    pub account_number: String,
    pin: String,
    x25519_secret: argus_x25519::EphemeralSecret,
    salt: String,
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
            x25519_secret: argus_x25519::generate_ephermeral_secret(),
            salt: argus_rand::generate_salt(),
        }
    }

    pub fn sha(&self) -> String {
        argus_sha2::digest_512::build_object(&self.pin, &self.salt)
    }

    pub fn public_key(&self) -> argus_x25519::PublicKey {
        argus_x25519::generate_public_key(&self.x25519_secret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_user() {
        let mut users: Vec<&str> = Vec::new();
        let test_user: User = User::new();
        let test_system: User = User::new();
        users.push(&test_user.account_number);
        users.push(&test_system.account_number);
        let test_user_sha = test_user.sha();
        let test_system_sha = test_system.sha();
        let test_user_public = test_user.public_key();
        let test_system_public = test_system.public_key();
        let test_user_shared_secret = test_user.x25519_secret.diffie_hellman(&test_system_public);
        let test_system_shared_secret = test_system.x25519_secret.diffie_hellman(&test_user_public);
        assert_ne!(test_user_sha, test_system_sha);
        assert_eq!(
            test_user_shared_secret.as_bytes(),
            test_system_shared_secret.as_bytes()
        );
        assert_eq!(users.len(), 2)
    }
}
