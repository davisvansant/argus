use rand::rngs::OsRng;
use rand::Rng;

pub fn generate_salt() -> String {
    let pool: &[u8] = b"\
    ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789\
    ";

    let salt_length: usize = 16;
    let mut rng = rand::thread_rng();
    let salt: String = (0..salt_length)
        .map(|_| {
            let index = rng.gen_range(0, pool.len());
            pool[index] as char
        })
        .collect();
    salt
}

pub fn generate_osrng() -> OsRng {
    OsRng {}
}

pub fn generate_pin() -> String {
    let mut random = rand::thread_rng();
    let pin: String = random.gen_range(000000, 999999).to_string();
    pin
}

pub fn generate_account_number() -> String {
    let mut random = rand::thread_rng();
    let account_number: String = random.gen_range(00000000, 99999999).to_string();
    account_number
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

    #[test]
    fn generate_and_compare_pins() {
        let pin_one = generate_pin();
        let pin_two = generate_pin();
        assert_ne!(pin_one, pin_two);
    }

    #[test]
    fn generate_and_compare_account_numbers() {
        let account_one = generate_account_number();
        let account_two = generate_account_number();
        assert_ne!(account_one, account_two);
    }
}
