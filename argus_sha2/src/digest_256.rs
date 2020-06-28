use sha2::{Digest, Sha256};

pub fn build_sha256_object() -> String {
    let mut hasher = Sha256::new();
    let message = String::from("hello!");

    hasher.update(&message);

    let built_hash = hasher.finalize();

    hex::encode(&built_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder() {
        let hasher = sha2::Sha256::digest(b"hello!");
        let hex = hex::encode(&hasher);
        let sha = build_sha256_object();
        assert_eq!(hex, sha);
    }
}
