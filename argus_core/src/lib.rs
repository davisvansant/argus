pub mod account;
pub mod encrypt;
pub mod message;
pub mod session;
pub mod state;
pub mod system;
pub mod user;

#[cfg(test)]
mod tests {
    use crate::system::System;
    use crate::user::User;
    #[test]
    fn create_and_verify_core() {
        let test_system: System = System::init();
        let mut users: Vec<&str> = Vec::new();
        let test_user_one: User = User::new();
        let test_user_two: User = User::new();
        users.push(&test_user_one.account_number);
        users.push(&test_user_two.account_number);
        let test_user_one_sha = test_user_one.sha();
        let test_user_two_sha = test_user_two.sha();
        let test_user_one_public_key = test_user_one.public_key();
        let test_user_two_public_key = test_user_two.public_key();
        let test_system_public_key = test_system.x25519_public_key();
        let test_user_one_ed25519_public_key = test_user_one.ed25519_public_key();
        let test_user_two_ed25519_public_key = test_user_two.ed25519_public_key();
        let test_user_one_shared_secret =
            test_user_one.x25519_shared_secret(&test_system_public_key);
        let test_user_two_shared_secret =
            test_user_two.x25519_shared_secret(&test_system_public_key);
        let test_system_shared_secret_with_test_user_one =
            test_system.x25519_shared_secret(&test_user_one_public_key);
        let test_system_shared_secret_with_test_user_two =
            test_system.x25519_shared_secret(&test_user_two_public_key);
        let test_user_one_message: &[u8] = b"This is a test from user one";
        let test_user_two_message: &[u8] = b"This is a test from user two";
        let test_user_one_signature =
            test_user_one.sign(&test_user_one_message, test_user_one_ed25519_public_key);
        let test_user_two_signature =
            test_user_two.sign(&test_user_two_message, test_user_two_ed25519_public_key);
        assert_eq!(users.len(), 2);
        assert_ne!(test_user_one_sha, test_user_two_sha);
        assert_eq!(
            &test_user_one_shared_secret.as_bytes(),
            &test_system_shared_secret_with_test_user_one.as_bytes()
        );
        assert_eq!(
            &test_user_two_shared_secret.as_bytes(),
            &test_system_shared_secret_with_test_user_two.as_bytes()
        );
        assert_ne!(
            &test_user_one_shared_secret.as_bytes(),
            &test_user_two_shared_secret.as_bytes(),
        );
        assert!(System::verify_signature(
            &test_user_one_ed25519_public_key,
            &test_user_one_message,
            &test_user_one_signature,
        )
        .is_ok());
        assert!(System::verify_signature(
            &test_user_two_ed25519_public_key,
            &test_user_two_message,
            &test_user_two_signature,
        )
        .is_ok());
    }
}
