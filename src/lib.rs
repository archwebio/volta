use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

/// Generates a random password
pub fn random_password() -> String {
    let password: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    password
}
