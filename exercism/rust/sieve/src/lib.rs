pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
  let mut primes: Vec<u64> = (0..=upper_bound).collect();
  for i in 2..=upper_bound {
    if primes[i as usize] > 0 {
      for j in (i * 2..=upper_bound).step_by(i as usize) {
        primes[j as usize] = 0;
      }
    }
  }
  primes.into_iter().filter(|&p| p >= 2).collect()
}
