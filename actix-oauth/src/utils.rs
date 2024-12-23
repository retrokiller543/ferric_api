use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

/// Returns a random alphanumeric string of length `length`.
pub fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}
