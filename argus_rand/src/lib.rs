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
