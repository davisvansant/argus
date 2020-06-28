pub fn generate_salt() -> u64 {
    rand::random::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_and_compare_salts() {
        let salt_one = generate_salt();
        let salt_two = generate_salt();
        assert_ne!(salt_one, salt_two);
    }
}
