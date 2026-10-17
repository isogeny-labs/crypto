use crypto_lib::utils::random::try_random_bytes_with_rng;
use rand::{rngs::SmallRng, SeedableRng};

fn main() {
    let mut rng = SmallRng::seed_from_u64(7);
    try_random_bytes_with_rng(&mut rng, &mut [0u8; 32]).unwrap();
}
