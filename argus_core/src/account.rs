use std::collections::HashMap;

pub struct Account {
    pub id: String,
}

impl Account {
    pub fn generate() -> Account {
        Account {
            id: argus_rand::generate_account_number(),
        }
    }

    pub fn information(&self) -> HashMap<String, String> {
        let pin = argus_rand::generate_pin();
        let salt = argus_rand::generate_salt();
        let sha = argus_sha2::digest_512::build_object(&pin, &salt);
        let mut account_information: HashMap<String, String> = HashMap::new();

        println!("[ argus ] Account number - {}", self.id);
        println!("[ argus ] PIN for account - {}", pin);

        account_information.insert(String::from("salt"), salt);
        account_information.insert(String::from("sha"), sha);

        account_information
    }

    pub fn secrets() -> Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>> {
        let secrets: Vec<HashMap<argus_uuid::Uuid, HashMap<String, argus_ed25519::Signature>>> =
            Vec::new();
        secrets
    }
}
