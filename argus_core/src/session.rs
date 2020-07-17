pub struct Session {
    x25519_static_secret: argus_x25519::StaticSecret,
}

impl Session {
    pub fn init() -> Self {
        Self {
            x25519_static_secret: argus_x25519::generate_static_secret(),
        }
    }

    pub fn x25519_public_key(&self) -> argus_x25519::PublicKey {
        argus_x25519::generate_public_key_from_static_secret(&self.x25519_static_secret)
    }

    pub fn x25519_shared_secret(
        &self,
        public_key: &argus_x25519::PublicKey,
    ) -> argus_x25519::SharedSecret {
        self.x25519_static_secret.diffie_hellman(public_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_session() {
        let test_system: Session = Session::init();
        let test_user: Session = Session::init();
        let test_system_x25519_public_key = test_system.x25519_public_key();
        let test_user_x25519_public_key = test_user.x25519_public_key();
        let test_system_shared_secret =
            test_system.x25519_shared_secret(&test_user_x25519_public_key);
        let test_user_shared_secret =
            test_user.x25519_shared_secret(&test_system_x25519_public_key);
        assert_eq!(
            test_user_shared_secret.as_bytes(),
            test_system_shared_secret.as_bytes()
        );
    }
}
