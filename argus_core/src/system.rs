use crate::user::User;
use std::collections::HashMap;

pub struct System {
    x25519_secret: argus_x25519::StaticSecret,
    // accounts: Vec<String>,
    private_account_information: HashMap<String, User>,
    pub public_account_information:
        HashMap<String, HashMap<String, crate::user::AccountInformation>>,
}

impl System {
    pub fn init() -> Self {
        Self {
            x25519_secret: argus_x25519::generate_static_secret(),
            // accounts: Vec::new(),
            private_account_information: HashMap::new(),
            public_account_information: HashMap::new(),
        }
    }

    pub fn x25519_public_key(&self) -> argus_x25519::PublicKey {
        argus_x25519::generate_public_key_from_static_secret(&self.x25519_secret)
    }

    pub fn x25519_shared_secret(
        &self,
        public_key: &argus_x25519::PublicKey,
    ) -> argus_x25519::SharedSecret {
        self.x25519_secret.diffie_hellman(public_key)
    }

    // pub fn secrets() -> HashMap<String, &'static [u8]> {
    //     let hash_map: HashMap<String, &'static [u8]> = HashMap::new();
    //     hash_map
    // }

    pub fn verify_signature(
        public_key: &argus_ed25519::PublicKey,
        message: &[u8],
        signature: &argus_ed25519::Signature,
    ) -> Result<(), argus_ed25519::SignatureError> {
        Ok(argus_ed25519::PublicKey::verify_strict(
            public_key, message, signature,
        )?)
    }

    pub fn save_private_account_information(&mut self, user: User) {
        let user_account_number = &user.account_number;
        self.private_account_information
            .insert(user_account_number.to_string(), user);

        println!("{:?}", self.private_account_information.len());
    }

    pub fn save_public_account_information(
        &mut self,
        user: &User,
        stuff: HashMap<String, crate::user::AccountInformation>,
    ) {
        let user_account_number = &user.account_number;

        self.public_account_information
            .insert(user_account_number.to_string(), stuff);
    }

    pub fn load_private_account_information(&mut self, account_id: &str) -> &mut User {
        let user_account_number = account_id;
        self.private_account_information
            .get_mut(user_account_number)
            .expect("not found") //panics on error...will need to fix later
    }

    // pub fn load_public_account_information(&mut self, account_id: &String) {
    //     let user_account_number = account_id;
    //     let _user_public_information: HashMap<String, crate::user::AccountInformation> =
    //         HashMap::new();
    //     let mut data = self
    //         .public_account_information
    //         .get_mut(user_account_number)
    //         .expect("not found"); //panics on error...will need to fix later
    //     let sha = &data.get_mut(&String::from("sha"));
    //     let x25519_public_key = &data.get_mut(&String::from("x25519_public_key"));
    //     let ed25519_public_key = &data.get_mut(&String::from("ed25519_public_key"));
    //     // println!("{:?}", sha);
    //     // println!("{:?}", x25519_public_key);
    //     // println!("{:?}", ed25519_public_key);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_system() {
        let test_system: System = System::init();
        let test_user: crate::user::User = crate::user::User::new();
        let test_system_public = test_system.x25519_public_key();
        let test_user_public = test_user.public_key();
        let test_system_shared_secret = test_system.x25519_shared_secret(&test_user_public);
        let test_user_shared_secret = test_user.x25519_shared_secret(&test_system_public);
        assert_eq!(
            test_user_shared_secret.as_bytes(),
            test_system_shared_secret.as_bytes()
        );
    }
}
