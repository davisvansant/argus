use std::collections::HashMap;

pub struct State {
    account_salt_and_sha: HashMap<String, HashMap<String, String>>,
    account_secrets: HashMap<String, HashMap<String, &'static [u8]>>,
}

impl State {
    pub fn init() -> Self {
        Self {
            account_salt_and_sha: HashMap::new(),
            account_secrets: HashMap::new(),
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
