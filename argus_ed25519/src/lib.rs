use ed25519_dalek::{Keypair, PublicKey, Signature};

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
}
