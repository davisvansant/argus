use sha2::{Digest, Sha512};

pub fn build_object(password: &str, salt: &str) -> String {
    let mut hasher = Sha512::new();

    hasher.update(password);
    hasher.update(salt);

    let built_hash = hasher.finalize();

    hex::encode(built_hash)
}

#[cfg(test)]
mod tests {
    use super::*;
    use argus_rand::generate_salt;
    #[test]
    fn generate_objects_and_compare() {
        let password_one: String = String::from("password");
        let password_two: String = String::from("password");
        let salt_one = generate_salt();
        let salt_two = generate_salt();
        let sha_one = build_object(&password_one, &salt_one);
        let sha_two = build_object(&password_two, &salt_two);
        let sha_three = build_object(&password_two, &salt_one);
        assert_ne!(sha_one, sha_two);
        assert_eq!(sha_one, sha_three);
    }
}
