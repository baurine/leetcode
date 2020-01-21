fn is_prime(num: u32) -> bool {
  for i in 2..(num / 2 + 1) {
    if num % i == 0 {
      return false;
    }
  }
  true
}

struct Primes(u32);

impl Iterator for Primes {
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    for i in (self.0 + 1).. {
      if is_prime(i) {
        self.0 = i;
        return Some(i);
      }
    }
    None
  }
}

pub fn nth(n: u32) -> u32 {
  Primes(1).nth(n as usize).unwrap()
}
