pub mod system;

pub struct User {
    pub account_number: String,
    pin: String,
    x25519_secret: argus_x25519::StaticSecret,
    salt: String,
    ed25519_secret_key: argus_ed25519::ExpandedSecretKey,
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
            // x25519_secret: argus_x25519::generate_ephermeral_secret(),
            x25519_secret: argus_x25519::generate_static_secret(),
            salt: argus_rand::generate_salt(),
            ed25519_secret_key: argus_ed25519::generate_expanded_secret_key(),
        }
    }

    pub fn sha(&self) -> String {
        argus_sha2::digest_512::build_object(&self.pin, &self.salt)
    }

    pub fn public_key(&self) -> argus_x25519::PublicKey {
        argus_x25519::generate_public_key_from_static_secret(&self.x25519_secret)
    }

    pub fn x25519_shared_secret(
        &self,
        public_key: &argus_x25519::PublicKey,
    ) -> argus_x25519::SharedSecret {
        self.x25519_secret.diffie_hellman(public_key)
    }

    pub fn ed25519_public_key(&self) -> argus_ed25519::PublicKey {
        argus_ed25519::generate_public_key_from_secret_key(&self.ed25519_secret_key)
    }

    pub fn sign(
        &self,
        message: &[u8],
        public_key: argus_ed25519::PublicKey,
    ) -> argus_ed25519::Signature {
        argus_ed25519::ExpandedSecretKey::sign(&self.ed25519_secret_key, &message, &public_key)
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
        let test_user_ed25519_public_key = test_user.ed25519_public_key();
        let test_system_ed25519_public_key = test_system.ed25519_public_key();
        let test_user_shared_secret = test_user.x25519_shared_secret(&test_system_public);
        let test_system_shared_secret = test_system.x25519_shared_secret(&test_user_public);
        let user_message: &[u8] = b"This is a test from user";
        let system_message: &[u8] = b"This is a test from system";
        let test_user_signature = test_user.sign(&user_message, test_user_ed25519_public_key);
        let test_system_signature =
            test_system.sign(&system_message, test_system_ed25519_public_key);
        assert!(argus_ed25519::PublicKey::verify(
            &test_user_ed25519_public_key,
            &user_message,
            &test_user_signature,
        )
        .is_ok());
        assert!(argus_ed25519::PublicKey::verify(
            &test_system_ed25519_public_key,
            &system_message,
            &test_system_signature,
        )
        .is_ok());
        assert_ne!(test_user_sha, test_system_sha);
        assert_eq!(
            &test_user_shared_secret.as_bytes(),
            &test_system_shared_secret.as_bytes()
        );
        assert_eq!(users.len(), 2)
    }
}
