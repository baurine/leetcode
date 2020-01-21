pub fn factors(n: u64) -> Vec<u64> {
  let mut leaving = n;
  let mut factor = 2;
  let mut factors_v = vec![];

  while leaving > 1 {
    if leaving % factor == 0 {
      factors_v.push(factor);
      leaving /= factor;
    } else {
      factor += 1;
    }
  }
  factors_v
}
