use crate::user::User;
use std::collections::HashMap;

pub enum AccountInfo {
    User(User),
    Sha(String),
    X25519PublicKey(argus_x25519::PublicKey),
    Ed25519PublicKey(argus_ed25519::PublicKey),
    SharedSecret(String),
}

pub struct System {
    x25519_secret: argus_x25519::StaticSecret,
}

impl System {
    pub fn init() -> Self {
        Self {
            x25519_secret: argus_x25519::generate_static_secret(),
        }
    }

    pub fn x25519_public_key(&self) -> argus_x25519::PublicKey {
        argus_x25519::generate_public_key_from_static_secret(&self.x25519_secret)
    }

    pub fn x25519_shared_secret(
        &self,
        public_key: &argus_x25519::PublicKey,
    ) -> argus_x25519::SharedSecret {
        self.x25519_secret.diffie_hellman(public_key)
    }

    pub fn secrets() -> HashMap<String, &'static [u8]> {
        let hash_map: HashMap<String, &'static [u8]> = HashMap::new();
        hash_map
    }

    pub fn verify_signature(
        public_key: &argus_ed25519::PublicKey,
        message: &[u8],
        signature: &argus_ed25519::Signature,
    ) -> Result<(), argus_ed25519::SignatureError> {
        Ok(argus_ed25519::PublicKey::verify(
            public_key, message, signature,
        )?)
    }

    pub fn capture_account_information(
        user: User,
        sha: String,
        public_key: argus_x25519::PublicKey,
        ed25519_public_key: argus_ed25519::PublicKey,
    ) -> HashMap<String, AccountInfo> {
        let mut account_information: HashMap<String, AccountInfo> = HashMap::new();
        account_information.insert(
            String::from("Account"),
            AccountInfo::Sha(user.account_number),
        );
        account_information.insert(String::from("sha"), AccountInfo::Sha(sha));
        account_information.insert(
            String::from("x25519_public_key"),
            AccountInfo::X25519PublicKey(public_key),
        );
        account_information.insert(
            String::from("ed25519_public_key"),
            AccountInfo::Ed25519PublicKey(ed25519_public_key),
        );
        account_information
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_system() {
        let test_system: System = System::init();
        let test_user: crate::user::User = crate::user::User::new();
        let test_system_public = test_system.x25519_public_key();
        let test_user_public = test_user.public_key();
        let test_system_shared_secret = test_system.x25519_shared_secret(&test_user_public);
        let test_user_shared_secret = test_user.x25519_shared_secret(&test_system_public);
        assert_eq!(
            test_user_shared_secret.as_bytes(),
            test_system_shared_secret.as_bytes()
        );
    }
}
