use x25519_dalek::{EphemeralSecret, PublicKey};

pub fn generate_ephermeral_secret() -> EphemeralSecret {
    let mut rng = argus_rand::generate_osrng();
    x25519_dalek::EphemeralSecret::new(&mut rng)
}

pub fn generate_public_key(secret: &EphemeralSecret) -> PublicKey {
    x25519_dalek::PublicKey::from(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compare_dh_keys() {
        let alice_secret = generate_ephermeral_secret();
        let bob_secret = generate_ephermeral_secret();
        let alice_public = generate_public_key(&alice_secret);
        let bob_public = generate_public_key(&bob_secret);
        let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
        let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
        assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
    }
}
