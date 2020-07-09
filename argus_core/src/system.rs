use std::collections::HashMap;

pub struct System {
    // probably needs to change to some static secret
    x25519_secret: argus_x25519::EphemeralSecret,
}

impl System {
    pub fn init() -> Self {
        Self {
            x25519_secret: argus_x25519::generate_ephermeral_secret(),
        }
    }

    pub fn x25519_public_key(&self) -> argus_x25519::PublicKey {
        argus_x25519::generate_public_key(&self.x25519_secret)
    }

    pub fn x25519_shared_secret(
        self,
        public_key: &argus_x25519::PublicKey,
    ) -> argus_x25519::SharedSecret {
        self.x25519_secret.diffie_hellman(public_key)
    }

    pub fn secrets() -> HashMap<String, &'static [u8]> {
        let hash_map: HashMap<String, &'static [u8]> = HashMap::new();
        hash_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::User;
    #[test]
    fn create_and_verify_system() {
        let test_system: System = System::init();
        let test_user: User = User::new();
        let test_system_public = test_system.x25519_public_key();
        let test_user_public = test_user.public_key();
        let test_user_secrets = System::secrets();
        let test_system_shared_secret = test_system.x25519_shared_secret(&test_user_public);
        let test_user_shared_secret = test_user.x25519_secret.diffie_hellman(&test_system_public);
        assert_eq!(
            test_user_shared_secret.as_bytes(),
            test_system_shared_secret.as_bytes()
        );
        assert_eq!(test_user_secrets.len(), 0)
    }
}
