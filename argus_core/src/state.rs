use crate::account::Account;
// use crate::message::MessageBundle;
use std::collections::HashMap;

type Secrets =
    HashMap<String, Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>>>;

pub struct State {
    pub account_salt_and_sha: HashMap<String, HashMap<String, String>>,
    // account_secrets:
    //     HashMap<String, HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>>,
    account_secrets: Secrets,
    // account_messages: HashMap<String, argus_uuid::Uuid>,
    // messages: Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>>,
}

impl State {
    pub fn init() -> Self {
        Self {
            account_salt_and_sha: HashMap::new(),
            account_secrets: HashMap::new(),
        }
    }

    pub fn save_account_information(
        &mut self,
        account: &Account,
        account_information: HashMap<String, String>,
        account_secrets: Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>>,
    ) {
        self.account_salt_and_sha
            .insert(account.id.to_string(), account_information);
        self.account_secrets
            .insert(account.id.to_string(), account_secrets);
    }

    pub fn load_account_information(&mut self, account: &str) -> &mut HashMap<String, String> {
        if self.account_salt_and_sha.contains_key(&account.to_string()) {
            self.account_salt_and_sha
                .get_mut(&account.to_string())
                .unwrap()
        } else {
            println!("account not found!");
            self.account_salt_and_sha
                .get_mut(&account.to_string())
                .unwrap() // need to handle - this panics
        }
    }

    pub fn verify_pin(&self, account: &str, pin: &str) -> bool {
        let account_information = self.account_salt_and_sha.get(&account.to_string()).unwrap();
        let current_salt = &account_information.get(&String::from("salt")).unwrap();
        let current_sha = &account_information.get(&String::from("sha")).unwrap();
        let computed_sha = argus_sha2::digest_512::build_object(&pin.to_string(), current_salt);
        *current_sha.to_string() == computed_sha
    }

    // pub fn save_account_secrets(
    //     &mut self,
    //     account: &Account,
    //     // secrets: HashMap<String, argus_ed25519::Signature>,
    //     secrets: Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>>,
    // ) {
    //     self.account_secrets.insert(account.id.to_string(), secrets);
    // }

    pub fn load_account_secrets(
        &mut self,
        account: &str,
        // ) -> &mut HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>> {
    ) -> &mut Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>> {
        self.account_secrets.get_mut(account).unwrap()
    }

    //
    // pub fn verify_message_and_save(
    //     &mut self,
    //     account: &str,
    //     ed25519_public_key: argus_ed25519::PublicKey,
    //     message: &[u8],
    //     signature: argus_ed25519::Signature,
    // ) {
    //     // let mut bundle_contents: HashMap<
    //     //     argus_uuid::Uuid,
    //     //     HashMap<String, argus_ed25519::Signature>,
    //     // > = HashMap::new();
    //     let mut bundle_contents: MessageBundle = HashMap::new();
    //     let mut bundle: Vec<MessageBundle> = Vec::new();
    //     let mut hash_map: HashMap<String, argus_ed25519::Signature> = HashMap::new();
    //     let public_key: argus_ed25519::PublicKey = ed25519_public_key;
    //     let message_uuid: argus_uuid::Uuid = argus_uuid::generate();
    //     // let mut vec = Vec::new();
    //     if public_key.verify(message, &signature).is_ok() {
    //         println!("looks good... adding");
    //         let converted_message = String::from_utf8_lossy(&message);
    //         hash_map.insert(converted_message.to_string(), signature);
    //         bundle_contents.insert(message_uuid, hash_map);
    //         // vec.push(bundle);
    //         bundle.push(bundle_contents);
    //         self.account_secrets.insert(account.to_string(), bundle);
    //     }

    pub fn save_message(
        &mut self,
        account: &str,
        bundle: HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>,
    ) {
        // let mut messages: Vec<
        //     HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>,
        // > = Vec::new();
        // messages.push(bundle);
        // self.account_secrets.insert(account.to_string(), messages);
        let another_m = self.account_secrets.get_mut(account).unwrap();
        // self.account_secrets
        //     .entry(account.to_string())
        //     .or_insert(messages);
        // self.account_secrets.iter_mut()
        // for value in self.account_secrets.values_mut() {
        //     // *value = messages;
        //     *value = messages.push(bundle).is_ok().to_vec();
        // }
        another_m.push(bundle);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_and_verify_state() {
        let test_state: State = State::init();
        assert_eq!(test_state.account_salt_and_sha.len(), 0);
        assert_eq!(test_state.account_secrets.len(), 0);
    }
}
