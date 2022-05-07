use rand;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Creates the random string of the given length.
pub fn create_nonce(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let nonce = (0..length)
        .map(|_| CHARS[rng.gen_range(0..CHARS.len())] as char)
        .collect();
    nonce
}

/// Generates the current timestamp in milliseconds.
pub fn get_current_timestamp() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}
