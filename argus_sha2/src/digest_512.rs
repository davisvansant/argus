use sha2::{Digest, Sha512};

pub fn build_object() -> String {
    let mut hasher = Sha512::new();
    let message = String::from("hello!");

    hasher.update(&message);

    let built_hash = hasher.finalize();

    hex::encode(&built_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_objects_and_compare() {
        let hasher = sha2::Sha512::digest(b"hello!");
        let hex = hex::encode(&hasher);
        let sha = build_object();
        assert_eq!(hex, sha);
    }
}
