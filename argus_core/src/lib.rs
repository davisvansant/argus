pub mod system;
pub mod user;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::User;
    #[test]
    fn create_and_verify_core() {
        let mut users: Vec<&str> = Vec::new();
        let test_user: User = User::new();
        let test_system: User = User::new();
        users.push(&test_user.account_number);
        users.push(&test_system.account_number);
        let test_user_sha = test_user.sha();
        let test_system_sha = test_system.sha();
        let test_user_public = test_user.public_key();
        let test_system_public = test_system.public_key();
        let test_user_ed25519_public_key = test_user.ed25519_public_key();
        let test_system_ed25519_public_key = test_system.ed25519_public_key();
        let test_user_shared_secret = test_user.x25519_shared_secret(&test_system_public);
        let test_system_shared_secret = test_system.x25519_shared_secret(&test_user_public);
        let user_message: &[u8] = b"This is a test from user";
        let system_message: &[u8] = b"This is a test from system";
        let test_user_signature = test_user.sign(&user_message, test_user_ed25519_public_key);
        let test_system_signature =
            test_system.sign(&system_message, test_system_ed25519_public_key);
        assert!(system::System::verify_signature(
            &test_user_ed25519_public_key,
            &user_message,
            &test_user_signature,
        )
        .is_ok());
        assert!(system::System::verify_signature(
            &test_system_ed25519_public_key,
            &system_message,
            &test_system_signature,
        )
        .is_ok());
        assert_ne!(test_user_sha, test_system_sha);
        assert_eq!(
            &test_user_shared_secret.as_bytes(),
            &test_system_shared_secret.as_bytes()
        );
        assert_eq!(users.len(), 2)
    }
}
