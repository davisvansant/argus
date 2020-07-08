use ed25519_dalek::{ExpandedSecretKey, Keypair, PublicKey, SecretKey, Signature};

pub fn generate_expanded_secret_key() -> ExpandedSecretKey {
    let mut csprng = argus_rand::generate_osrng();
    let secret_key = SecretKey::generate(&mut csprng);
    let expanded_secret_key: ExpandedSecretKey = ExpandedSecretKey::from(&secret_key);
    expanded_secret_key
}

pub fn generate_public_key_from_secret_key(secret_key: &ExpandedSecretKey) -> PublicKey {
    let public_key: PublicKey = PublicKey::from(secret_key);
    public_key
}

pub fn generate_keypair() -> Keypair {
    let mut csprng = argus_rand::generate_osrng();
    Keypair::generate(&mut csprng)
}

pub fn generate_signature(keypair: &Keypair, message: &[u8]) -> Signature {
    keypair.sign(message)
}

pub fn generate_public_key(keypair: &Keypair) -> PublicKey {
    keypair.public
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_keypair_and_verify() {
        let keypair = generate_keypair();
        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature = generate_signature(&keypair, &message);
        let public_key = generate_public_key(&keypair);
        assert!(&keypair.verify(&message, &signature).is_ok());
        assert!(&public_key.verify(&message, &signature).is_ok());
    }

    #[test]
    fn generate_keypair_from_public_key_from_secret_key_and_verify() {
        let secret_key_one = generate_expanded_secret_key();
        let secret_key_two = generate_expanded_secret_key();
        let public_key_one = generate_public_key_from_secret_key(&secret_key_one);
        let public_key_two = generate_public_key_from_secret_key(&secret_key_two);
        let message: &[u8] = b"This is a test of the tsunami alert system.";
        let signature_one = ExpandedSecretKey::sign(&secret_key_one, &message, &public_key_one);
        let signature_two = ExpandedSecretKey::sign(&secret_key_two, &message, &public_key_two);
        assert!(PublicKey::verify(&public_key_one, &message, &signature_one).is_ok());
        assert!(PublicKey::verify(&public_key_two, &message, &signature_two).is_ok());
    }
}
