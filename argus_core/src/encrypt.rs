use argus_aes::gcm::Aead;

pub struct Encrypt {
    cipher: argus_aes::gcm::Aes256Gcm,
    nonce: argus_aes::gcm::Nonce,
}

impl Encrypt {
    pub fn init() -> Self {
        Self {
            cipher: argus_aes::gcm::generate_cipher(),
            nonce: argus_aes::gcm::generate_nonce(),
        }
    }

    pub fn encrypt_data(&self) -> Vec<u8> {
        let cipher = &self.cipher;
        cipher
            .encrypt(&self.nonce, b"something to encrypt!".as_ref())
            .expect("encryption failure!")
    }

    pub fn decrypt_data(&self, ciphertext: &[u8]) -> Vec<u8> {
        let cipher = &self.cipher;
        cipher
            .decrypt(&self.nonce, ciphertext.as_ref())
            .expect("decryption failure!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encypt_and_verify() {
        let encryption = Encrypt::init();
        let ciphertext = encryption.encrypt_data();
        let plaintext = encryption.decrypt_data(&ciphertext);
        assert_eq!(&plaintext, b"something to encrypt!");
    }
}
