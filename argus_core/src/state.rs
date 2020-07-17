use crate::account::Account;
use std::collections::HashMap;

pub struct State {
    pub account_salt_and_sha: HashMap<String, HashMap<String, String>>,
    account_secrets: HashMap<String, HashMap<String, &'static [u8]>>,
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
    ) {
        self.account_salt_and_sha
            .insert(account.id.to_string(), account_information);
    }

    pub fn load_account_information(&mut self, account: &str) -> &mut HashMap<String, String> {
        if self.account_salt_and_sha.contains_key(&account.to_string()) {
            println!("loading account - {:?}", account);
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
