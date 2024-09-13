use base64::encode;
use rand::{self, distributions::DistString};
use rand::distributions::Alphanumeric;
use sha2::{Digest, Sha384, Sha512};
const SECRET_KEY: &'static str = "xunfei123";

pub fn sha256_hash(str: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(str);
    hasher.update(SECRET_KEY);
    let encrypted_password = hasher.finalize();
    encode(encrypted_password)
}

pub fn get_random_str() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 6)
}
