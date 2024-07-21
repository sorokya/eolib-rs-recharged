use rand::Rng;

/// returns a random swap multiple
pub fn generate_encryption_key() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(100..=10000)
}
