pub use aes_gcm::aead::generic_array::typenum::consts::U12;
pub use aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
pub use aes_gcm::Aes256Gcm;

pub type Nonce = GenericArray<u8, U12>;

pub fn generate_cipher() -> Aes256Gcm {
    let key = argus_rand::generate_aes_key();
    let key_slice = key.as_bytes();
    let cipher_key = GenericArray::from_slice(key_slice);
    Aes256Gcm::new(cipher_key)
}

pub fn generate_nonce() -> Nonce {
    let generate_nonce = argus_rand::generate_nonce();
    let nonce_slice = generate_nonce.as_bytes();
    *GenericArray::from_slice(nonce_slice)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_generic_cipher_and_verify() {
        let key = GenericArray::from_slice(b"i will be 32 bytes in length!!!!");
        let cipher = Aes256Gcm::new(key);
        let nonce = GenericArray::from_slice(b"i will be 12");
        let ciphertext = cipher
            .encrypt(nonce, b"something to encrypt!".as_ref())
            .expect("encryption failure!");
        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .expect("decryption failure!");
        assert_eq!(&plaintext, b"something to encrypt!");
    }
    #[test]
    fn create_cipher_and_verify() {
        let cipher = generate_cipher();
        let nonce = generate_nonce();
        let ciphertext = cipher
            .encrypt(&nonce, b"something to encrypt!".as_ref())
            .expect("encryption failure!");
        let plaintext = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .expect("decryption failure!");
        assert_eq!(&plaintext, b"something to encrypt!");
    }
}
