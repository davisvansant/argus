pub use x25519_dalek::{EphemeralSecret, PublicKey, SharedSecret, StaticSecret};

pub fn generate_ephermeral_secret() -> EphemeralSecret {
    let mut rng = argus_rand::generate_osrng();
    x25519_dalek::EphemeralSecret::new(&mut rng)
}

pub fn generate_static_secret() -> StaticSecret {
    let mut rng = argus_rand::generate_osrng();
    x25519_dalek::StaticSecret::new(&mut rng)
}

pub fn generate_public_key_from_ephemeral_secret(secret: &EphemeralSecret) -> PublicKey {
    x25519_dalek::PublicKey::from(secret)
}

pub fn generate_public_key_from_static_secret(secret: &StaticSecret) -> PublicKey {
    x25519_dalek::PublicKey::from(secret)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compare_dh_keys() {
        let alice_secret = generate_ephermeral_secret();
        let bob_secret = generate_ephermeral_secret();
        let alice_public = generate_public_key_from_ephemeral_secret(&alice_secret);
        let bob_public = generate_public_key_from_ephemeral_secret(&bob_secret);
        let alice_shared_secret = alice_secret.diffie_hellman(&bob_public);
        let bob_shared_secret = bob_secret.diffie_hellman(&alice_public);
        assert_eq!(alice_shared_secret.as_bytes(), bob_shared_secret.as_bytes());
    }
}
