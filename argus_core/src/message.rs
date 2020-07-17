pub struct Message {
    ed25519_secret_key: argus_ed25519::ExpandedSecretKey,
}

impl Message {
    pub fn prepare() -> Self {
        Self {
            ed25519_secret_key: argus_ed25519::generate_expanded_secret_key(),
        }
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
    fn create_and_verify_message() {
        let test_message: Message = Message::prepare();
        let test_message_ed25519_public_key = test_message.ed25519_public_key();
        let test_message_contents: &[u8] = b"This is a test from user";
        let test_message_signature =
            test_message.sign(&test_message_contents, test_message_ed25519_public_key);
        assert!(test_message_ed25519_public_key
            .verify(test_message_contents, &test_message_signature)
            .is_ok());
    }
}
