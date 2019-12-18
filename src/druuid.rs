use rand::Rng;
use chrono::{DateTime, Utc};
use num_traits::pow;

const RANDOM_BITS: u64 = 23;

pub fn gen_druuid(datetime: DateTime<Utc>) -> u64 {
  let time_bits: u64 = (datetime.timestamp() as u64 * 1000) << RANDOM_BITS;
  time_bits | randomness()
}

fn randomness() -> u64 {
  let mut rng = rand::thread_rng();
  let rand: u64 = rng.gen_range(0, 1000000000000000);
  let modulus: u64 = pow(2u64, RANDOM_BITS as usize);
  (rand % modulus)
}
