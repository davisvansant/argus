pub use uuid::Uuid;

pub fn generate() -> Uuid {
    Uuid::new_v4()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_uuid_and_compare() {
        let uuid_one = crate::generate();
        let uuid_two = crate::generate();
        let nil = Uuid::nil();
        assert_ne!(uuid_one, nil);
        assert_ne!(uuid_one, uuid_two);
    }
}
